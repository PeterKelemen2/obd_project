use std::io::{Read, Write};
use std::time::Duration;

use serialport::SerialPort;

use crate::command::{pids, ObdCommand};
use crate::error::ObdError;
use crate::pid::{decode_coolant_temp, decode_engine_rpm, decode_vehicle_speed, PidDecode};
use crate::response::parse_hex_bytes;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default baud rate used when opening the serial port.
pub const DEFAULT_BAUD_RATE: u32 = 38_400;

/// Default read timeout.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// The ELM327 ready prompt character.
const ELM_PROMPT: u8 = b'>';

/// Maximum bytes to read in a single `send_command` call.
const READ_BUF_SIZE: usize = 256;

// ---------------------------------------------------------------------------
// ELM327 initialisation commands (AT commands)
// ---------------------------------------------------------------------------

/// AT commands sent during [`ObdConnection::initialize`].
const INIT_COMMANDS: &[&str] = &[
    "ATZ\r",  // Reset all
    "ATE0\r", // Echo off
    "ATL0\r", // Linefeeds off
    "ATS0\r", // Spaces off  (some builds use spaces; keep for compatibility)
    "ATH0\r", // Headers off
    "ATSP0\r",
    "ATRV\r",
];

// ---------------------------------------------------------------------------
// SerialConnection – low-level serial wrapper
// ---------------------------------------------------------------------------

/// Low-level wrapper around a `serialport` serial port handle.
///
/// Manages opening the port, writing commands, and reading responses up to
/// the ELM327 `>` prompt character.
pub struct SerialConnection {
    port: Box<dyn SerialPort>,
}

impl SerialConnection {
    /// Open a serial connection to `path` at the given `baud_rate`.
    ///
    /// # Errors
    ///
    /// Returns [`ObdError::Serial`] if the port cannot be opened.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use obd2_reader::connection::SerialConnection;
    ///
    /// let conn = SerialConnection::connect("/dev/rfcomm1", 38400).unwrap();
    /// ```
    pub fn connect(path: &str, baud_rate: u32) -> Result<Self, ObdError> {
        let port = serialport::new(path, baud_rate)
            .timeout(DEFAULT_TIMEOUT)
            .open()
            .map_err(ObdError::Serial)?;

        Ok(Self { port })
    }

    /// Send an AT or OBD command string and return the adapter's response as a
    /// trimmed `String`.
    ///
    /// The method:
    /// 1. Writes `cmd` bytes directly to the port.
    /// 2. Reads bytes until the ELM327 `>` prompt is received or the buffer is full.
    /// 3. Strips the trailing prompt and returns the trimmed response.
    ///
    /// # Errors
    ///
    /// * [`ObdError::Io`] – if writing or reading the port fails.
    /// * [`ObdError::Timeout`] – if no `>` prompt is received within the timeout.
    // Increase timeout — SEARCHING can take several seconds
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

    pub fn send_command(&mut self, cmd: &str) -> Result<String, ObdError> {
        self.port.flush()?;
        self.port.write_all(cmd.as_bytes())?;

        let mut response = Vec::with_capacity(READ_BUF_SIZE);
        let mut buf = [0u8; 1];

        loop {
            match self.port.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    if buf[0] == ELM_PROMPT {
                        break; // Only stop at the prompt — never timeout mid-response
                    }
                    response.push(buf[0]);
                    if response.len() >= READ_BUF_SIZE {
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(ObdError::Timeout); // Timeout only if we got nothing at all
                }
                Err(e) => return Err(ObdError::Io(e)),
            }
        }

        let text = String::from_utf8_lossy(&response).into_owned();
        println!("[DEBUG raw] {:?}", text);

        let result = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter(|l| !is_status_message(l))
            .collect::<Vec<_>>()
            .join(" ");

        Ok(result)
    }
}

fn is_status_message(line: &str) -> bool {
    let upper = line.to_uppercase();
    upper.starts_with("SEARCHING")
        || upper.starts_with("BUS INIT")
        || upper.starts_with("TRYING")
        || upper.starts_with("AUTO")
}
// ---------------------------------------------------------------------------
// ObdConnection – high-level API
// ---------------------------------------------------------------------------

/// High-level OBD-II connection that wraps [`SerialConnection`] and exposes
/// ergonomic methods for querying live vehicle data.
///
/// # Examples
///
/// ```no_run
/// use obd2_reader::connection::ObdConnection;
///
/// let mut obd = ObdConnection::connect("/dev/rfcomm1").unwrap();
/// obd.initialize().unwrap();
///
/// let rpm = obd.query_engine_rpm().unwrap();
/// println!("Engine RPM: {rpm}");
///
/// let speed = obd.query_vehicle_speed().unwrap();
/// println!("Vehicle speed: {speed} km/h");
///
/// let temp = obd.query_coolant_temp().unwrap();
/// println!("Coolant temperature: {temp} °C");
/// ```
pub struct ObdConnection {
    conn: SerialConnection,
}

impl ObdConnection {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// Open a connection to an ELM327 adapter at `path` using the default
    /// baud rate ([`DEFAULT_BAUD_RATE`] = 38 400).
    ///
    /// This does **not** run the initialisation sequence; call
    /// [`ObdConnection::initialize`] afterwards.
    ///
    /// # Errors
    ///
    /// Returns [`ObdError::Serial`] if the port cannot be opened.
    pub fn connect(path: &str) -> Result<Self, ObdError> {
        Self::connect_with_baud(path, DEFAULT_BAUD_RATE)
    }

    /// Open a connection at a custom baud rate.
    pub fn connect_with_baud(path: &str, baud_rate: u32) -> Result<Self, ObdError> {
        let conn = SerialConnection::connect(path, baud_rate)?;
        Ok(Self { conn })
    }

    // -----------------------------------------------------------------------
    // Initialisation
    // -----------------------------------------------------------------------

    /// Run the standard ELM327 initialization sequence.
    ///
    /// Sends `ATZ`, `ATE0`, `ATL0`, `ATS0`, and `ATH0` in order, checking
    /// that none of them triggers an adapter-error response.
    ///
    /// # Errors
    ///
    /// Returns [`ObdError::InitFailed`] if any command returns an unexpected
    /// response, or any lower-level error from the serial layer.
    pub fn initialize(&mut self) -> Result<(), ObdError> {
        for cmd in INIT_COMMANDS {
            let resp = self.conn.send_command(cmd)?;
            println!("[INIT] {} -> {:?}", cmd.trim(), resp);

            // Give the adapter time to process each command
            std::thread::sleep(std::time::Duration::from_millis(200));

            // ATZ needs extra time to fully reset
            if cmd.starts_with("ATZ") {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            let upper = resp.to_uppercase();
            for token in &["ERROR", "UNABLE TO CONNECT", "BUS ERROR"] {
                if upper.contains(token) {
                    return Err(ObdError::InitFailed(format!(
                        "Command '{cmd}' failed with: {resp}"
                    )));
                }
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Raw query helper
    // -----------------------------------------------------------------------

    /// Send an [`ObdCommand`] and return the parsed raw bytes.
    ///
    /// This is the building block used by all `query_*` methods.
    pub fn query_raw(&mut self, cmd: ObdCommand) -> Result<Vec<u8>, ObdError> {
        let response = self.conn.send_command(&cmd.to_command_string())?;
        parse_hex_bytes(&response)
    }

    /// Send an arbitrary AT or OBD string and return the raw response.
    ///
    /// Useful for ad-hoc commands or adapter configuration.
    pub fn send_raw(&mut self, cmd: &str) -> Result<String, ObdError> {
        self.conn.send_command(cmd)
    }

    pub fn query_supported_pids(&mut self) -> Result<Vec<u8>, ObdError> {
        use crate::pid::decode_supported_pids;

        // OBD-II splits supported PIDs into blocks of 32.
        // PID 0x00 = supported PIDs 01-20
        // PID 0x20 = supported PIDs 21-40
        // PID 0x40 = supported PIDs 41-60
        let range_bases: &[u8] = &[0x00, 0x20, 0x40];
        let mut all_supported = Vec::new();

        for &base in range_bases {
            let cmd = ObdCommand::new(0x01, base);
            let raw = self.query_raw(cmd)?;
            let pids = decode_supported_pids(&raw, base)?;

            // PID 0x20/0x40/0x60 being supported means the next range exists.
            // We use that to decide whether to keep querying.
            let has_next_range = pids.contains(&(base + 0x20));
            all_supported.extend(pids);

            if !has_next_range {
                break; // No more ranges supported, stop here
            }
        }

        Ok(all_supported)
    }

    // -----------------------------------------------------------------------
    // PID queries
    // -----------------------------------------------------------------------

    /// Query a PID and decode the result into any type that implements `PidDecode`.
    /// Rust infers which decoder to use from the expected return type.
    pub fn query<T: PidDecode>(&mut self, cmd: ObdCommand) -> Result<T, ObdError> {
        let raw = self.query_raw(cmd)?;
        T::decode(cmd.pid, &raw)
    }

    /// Query the current engine RPM.
    ///
    /// Returns engine speed in **RPM** as `f32`.
    ///
    /// # Errors
    ///
    /// * [`ObdError::Timeout`] – adapter did not respond.
    /// * [`ObdError::AdapterError`] – e.g. "NO DATA" (engine off).
    /// * [`ObdError::InsufficientData`] – unexpected short response.
    pub fn query_engine_rpm(&mut self) -> Result<f32, ObdError> {
        let raw = self.query_raw(pids::ENGINE_RPM)?;
        decode_engine_rpm(&raw)
    }

    /// Query the current vehicle speed.
    ///
    /// Returns speed in **km/h** as `u8`.
    pub fn query_vehicle_speed(&mut self) -> Result<u8, ObdError> {
        let raw = self.query_raw(pids::VEHICLE_SPEED)?;
        decode_vehicle_speed(&raw)
    }

    /// Query the engine coolant temperature.
    ///
    /// Returns temperature in **°C** as `i16` (can be negative at cold start).
    pub fn query_coolant_temp(&mut self) -> Result<i16, ObdError> {
        let raw = self.query_raw(pids::COOLANT_TEMP)?;
        decode_coolant_temp(&raw)
    }

    /// Query the current throttle position.
    ///
    /// Returns throttle as a percentage (`0.0` – `100.0`).
    pub fn query_throttle_position(&mut self) -> Result<f32, ObdError> {
        use crate::pid::decode_throttle_position;
        let raw = self.query_raw(pids::THROTTLE_POSITION)?;
        decode_throttle_position(&raw)
    }

    /// Query the fuel level.
    ///
    /// Returns fuel level as a percentage (`0.0` – `100.0`).
    pub fn query_fuel_level(&mut self) -> Result<f32, ObdError> {
        use crate::pid::decode_fuel_level;
        let raw = self.query_raw(pids::FUEL_LEVEL)?;
        decode_fuel_level(&raw)
    }
}
