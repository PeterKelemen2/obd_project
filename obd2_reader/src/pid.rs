use crate::error::ObdError;
use crate::response::extract_data_bytes;

/// Query which PIDs (01–20) the vehicle supports.
/// Returns a Vec of supported PID numbers.
pub fn decode_supported_pids(raw: &[u8], base_pid: u8) -> Result<Vec<u8>, ObdError> {
    let data = extract_data_bytes(raw, 4)?;

    // The 4 bytes form a 32-bit bitmask.
    // Bit 31 (MSB) = PID base+1, Bit 0 (LSB) = PID base+32
    let bitmask = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

    let mut supported = Vec::new();
    for bit in 0..32u8 {
        // bit 31 corresponds to base_pid+1, bit 0 to base_pid+32
        if bitmask & (1 << (31 - bit)) != 0 {
            supported.push(base_pid + bit + 1);
        }
    }

    Ok(supported)
}

pub trait PidDecode: Sized {
    fn decode(pid: u8, raw: &[u8]) -> Result<Self, ObdError>;
}

impl PidDecode for f32 {
    fn decode(pid: u8, raw: &[u8]) -> Result<Self, ObdError> {
        match pid {
            0x0C => decode_engine_rpm(raw),
            0x11 => decode_throttle_position(raw),
            0x2F => decode_fuel_level(raw),
            _ => Err(ObdError::Parse(format!(
                "No f32 decoder for PID {pid:#04X}"
            ))),
        }
    }
}

impl PidDecode for u8 {
    fn decode(pid: u8, raw: &[u8]) -> Result<Self, ObdError> {
        match pid {
            0x0D => decode_vehicle_speed(raw),
            _ => Err(ObdError::Parse(format!("No u8 decoder for PID {pid:#04X}"))),
        }
    }
}

impl PidDecode for i16 {
    fn decode(pid: u8, raw: &[u8]) -> Result<Self, ObdError> {
        match pid {
            0x05 => decode_coolant_temp(raw),
            _ => Err(ObdError::Parse(format!(
                "No i16 decoder for PID {pid:#04X}"
            ))),
        }
    }
}

// ---------------------------------------------------------------------------
// Individual PID decoders
// ---------------------------------------------------------------------------

/// Decode engine RPM from two raw data bytes.
///
/// Formula: `((A * 256) + B) / 4`
///
/// # Arguments
///
/// * `raw` – The complete response byte slice including the two echo bytes,
///   e.g. `[0x41, 0x0C, 0x1A, 0xF8]`.
///
/// # Returns
///
/// Engine speed in **RPM** as a `f32`.
///
/// # Examples
///
/// ```rust
/// use obd2_reader::pid::decode_engine_rpm;
///
/// // Raw response: 41 0C 1A F8
/// let bytes = vec![0x41, 0x0C, 0x1A, 0xF8];
/// let rpm = decode_engine_rpm(&bytes).unwrap();
/// assert_eq!(rpm, 1726.0); // ((0x1A * 256) + 0xF8) / 4
/// ```
pub fn decode_engine_rpm(raw: &[u8]) -> Result<f32, ObdError> {
    let data = extract_data_bytes(raw, 2)?;
    let a = data[0] as u32;
    let b = data[1] as u32;
    Ok(((a * 256) + b) as f32 / 4.0)
}

/// Decode vehicle speed from one raw data byte.
///
/// Formula: `A` (the value is already km/h).
///
/// # Arguments
///
/// * `raw` – The complete response byte slice including the two echo bytes,
///   e.g. `[0x41, 0x0D, 0x3C]`.
///
/// # Returns
///
/// Vehicle speed in **km/h** as a `u8`.
///
/// # Examples
///
/// ```rust
/// use obd2_reader::pid::decode_vehicle_speed;
///
/// // Raw response: 41 0D 3C  (60 km/h)
/// let bytes = vec![0x41, 0x0D, 0x3C];
/// let speed = decode_vehicle_speed(&bytes).unwrap();
/// assert_eq!(speed, 60);
/// ```
pub fn decode_vehicle_speed(raw: &[u8]) -> Result<u8, ObdError> {
    let data = extract_data_bytes(raw, 1)?;
    Ok(data[0])
}

/// Decode engine coolant temperature from one raw data byte.
///
/// Formula: `A - 40`
///
/// # Arguments
///
/// * `raw` – The complete response byte slice including the two echo bytes,
///   e.g. `[0x41, 0x05, 0x6E]`.
///
/// # Returns
///
/// Coolant temperature in **°C** as an `i16` (can be negative at cold start).
///
/// # Examples
///
/// ```rust
/// use obd2_reader::pid::decode_coolant_temp;
///
/// // Raw response: 41 05 6E  (70 °C)
/// let bytes = vec![0x41, 0x05, 0x6E];
/// let temp = decode_coolant_temp(&bytes).unwrap();
/// assert_eq!(temp, 70);
/// ```
pub fn decode_coolant_temp(raw: &[u8]) -> Result<i16, ObdError> {
    let data = extract_data_bytes(raw, 1)?;
    Ok(data[0] as i16 - 40)
}

/// Decode throttle position from one raw data byte.
///
/// Formula: `A * 100 / 255`
///
/// # Returns
///
/// Throttle position as a percentage (`0.0` – `100.0`).
pub fn decode_throttle_position(raw: &[u8]) -> Result<f32, ObdError> {
    let data = extract_data_bytes(raw, 1)?;
    Ok(data[0] as f32 * 100.0 / 255.0)
}

/// Decode fuel level input from one raw data byte.
///
/// Formula: `A * 100 / 255`
///
/// # Returns
///
/// Fuel level as a percentage (`0.0` – `100.0`).
pub fn decode_fuel_level(raw: &[u8]) -> Result<f32, ObdError> {
    let data = extract_data_bytes(raw, 1)?;
    Ok(data[0] as f32 * 100.0 / 255.0)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpm_decode() {
        // 41 0C 1A F8  =>  ((0x1A * 256) + 0xF8) / 4 = (6656 + 248) / 4 = 1726.0
        let raw = vec![0x41, 0x0C, 0x1A, 0xF8];
        assert_eq!(decode_engine_rpm(&raw).unwrap(), 1726.0);
    }

    #[test]
    fn speed_decode() {
        // 41 0D 3C  =>  0x3C = 60 km/h
        let raw = vec![0x41, 0x0D, 0x3C];
        assert_eq!(decode_vehicle_speed(&raw).unwrap(), 60);
    }

    #[test]
    fn coolant_decode() {
        // 41 05 6E  =>  0x6E (110) - 40 = 70 °C
        let raw = vec![0x41, 0x05, 0x6E];
        assert_eq!(decode_coolant_temp(&raw).unwrap(), 70);
    }

    #[test]
    fn coolant_negative() {
        // 41 05 00  =>  0 - 40 = -40 °C
        let raw = vec![0x41, 0x05, 0x00];
        assert_eq!(decode_coolant_temp(&raw).unwrap(), -40);
    }

    #[test]
    fn throttle_full_open() {
        // 41 11 FF  =>  255 * 100 / 255 = 100.0 %
        let raw = vec![0x41, 0x11, 0xFF];
        let pct = decode_throttle_position(&raw).unwrap();
        assert!((pct - 100.0).abs() < f32::EPSILON);
    }
}
