/// Represents a single OBD-II command composed of a *mode* byte and a *PID* byte.
///
/// # Examples
///
/// ```rust
/// use obd2_reader::command::ObdCommand;
///
/// let cmd = ObdCommand::new(0x01, 0x0C); // Engine RPM
/// assert_eq!(cmd.to_command_string(), "010C\r");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObdCommand {
    /// OBD-II service / mode (e.g. `0x01` for "current data").
    pub mode: u8,
    /// Parameter identifier within the chosen mode.
    pub pid: u8,
}

impl ObdCommand {
    /// Create a new [`ObdCommand`].
    ///
    /// # Arguments
    ///
    /// * `mode` – OBD-II service byte (e.g. `0x01`).
    /// * `pid`  – Parameter ID byte (e.g. `0x0C` for engine RPM).
    pub fn new(mode: u8, pid: u8) -> Self {
        Self { mode, pid }
    }

    /// Convert the command to the ASCII string expected by the ELM327 adapter.
    ///
    /// The format is two upper-case hex digits for the mode, two for the PID,
    /// followed by a carriage-return (`\r`) which acts as the line terminator for
    /// the ELM327 protocol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use obd2_reader::command::ObdCommand;
    ///
    /// assert_eq!(ObdCommand::new(0x01, 0x0C).to_command_string(), "010C\r");
    /// assert_eq!(ObdCommand::new(0x01, 0x0D).to_command_string(), "010D\r");
    /// assert_eq!(ObdCommand::new(0x01, 0x05).to_command_string(), "0105\r");
    /// ```
    pub fn to_command_string(&self) -> String {
        format!("{:02X}{:02X}\r", self.mode, self.pid)
    }
}

impl std::fmt::Display for ObdCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}", self.mode, self.pid)
    }
}

/// Well-known OBD-II PID constants (Mode 01 – current data).
pub mod pids {
    use super::ObdCommand;

    /// Supported PIDs 01-20
    pub const SUPPORTED_PIDS_01_20: ObdCommand = ObdCommand { mode: 0x01, pid: 0x00 };
    /// Engine coolant temperature
    pub const COOLANT_TEMP: ObdCommand = ObdCommand { mode: 0x01, pid: 0x05 };
    /// Engine RPM
    pub const ENGINE_RPM: ObdCommand = ObdCommand { mode: 0x01, pid: 0x0C };
    /// Vehicle speed
    pub const VEHICLE_SPEED: ObdCommand = ObdCommand { mode: 0x01, pid: 0x0D };
    /// Throttle position
    pub const THROTTLE_POSITION: ObdCommand = ObdCommand { mode: 0x01, pid: 0x11 };
    /// Fuel level input
    pub const FUEL_LEVEL: ObdCommand = ObdCommand { mode: 0x01, pid: 0x2F };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpm_command_string() {
        assert_eq!(pids::ENGINE_RPM.to_command_string(), "010C\r");
    }

    #[test]
    fn speed_command_string() {
        assert_eq!(pids::VEHICLE_SPEED.to_command_string(), "010D\r");
    }

    #[test]
    fn coolant_command_string() {
        assert_eq!(pids::COOLANT_TEMP.to_command_string(), "0105\r");
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", pids::ENGINE_RPM), "010C");
    }
}