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

/// OBD-II PID constants (Mode 01 – current data).
pub mod pids {
    use super::ObdCommand;

    // Supported PIDs 01-20
    pub const SUPPORTED_PIDS_01_20: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x00,
    };
    pub const STATUS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x01,
    };
    pub const FREEZE_DTC: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x02,
    };
    pub const FUEL_STATUS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x03,
    };
    pub const ENGINE_LOAD: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x04,
    };
    pub const COOLANT_TEMP: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x05,
    };
    pub const SHORT_FUEL_TRIM_1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x06,
    };
    pub const LONG_FUEL_TRIM_1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x07,
    };
    pub const SHORT_FUEL_TRIM_2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x08,
    };
    pub const LONG_FUEL_TRIM_2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x09,
    };
    pub const FUEL_PRESSURE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0A,
    };
    pub const INTAKE_PRESSURE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0B,
    };
    pub const ENGINE_RPM: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0C,
    };
    pub const VEHICLE_SPEED: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0D,
    };
    pub const TIMING_ADVANCE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0E,
    };
    pub const INTAKE_TEMP: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x0F,
    };
    pub const MAF: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x10,
    };
    pub const THROTTLE_POSITION: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x11,
    };
    pub const AIR_STATUS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x12,
    };
    pub const O2_SENSORS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x13,
    };

    // Supported PIDs 21-40
    pub const SUPPORTED_PIDS_21_40: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x20,
    };
    pub const DISTANCE_W_MIL: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x21,
    };
    pub const FUEL_RAIL_PRESSURE_VAC: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x22,
    };
    pub const FUEL_RAIL_PRESSURE_DIRECT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x23,
    };
    pub const O2_S1_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x24,
    };
    pub const O2_S2_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x25,
    };
    pub const O2_S3_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x26,
    };
    pub const O2_S4_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x27,
    };
    pub const O2_S5_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x28,
    };
    pub const O2_S6_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x29,
    };
    pub const O2_S7_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2A,
    };
    pub const O2_S8_WR_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2B,
    };
    pub const COMMANDED_EGR: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2C,
    };
    pub const EGR_ERROR: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2D,
    };
    pub const EVAPORATIVE_PURGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2E,
    };
    pub const FUEL_LEVEL: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x2F,
    };
    pub const WARMUPS_SINCE_DTC_CLEAR: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x30,
    };
    pub const DISTANCE_SINCE_DTC_CLEAR: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x31,
    };
    pub const EVAP_VAPOR_PRESSURE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x32,
    };
    pub const BAROMETRIC_PRESSURE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x33,
    };
    pub const O2_S1_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x34,
    };
    pub const O2_S2_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x35,
    };
    pub const O2_S3_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x36,
    };
    pub const O2_S4_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x37,
    };
    pub const O2_S5_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x38,
    };
    pub const O2_S6_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x39,
    };
    pub const O2_S7_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3A,
    };
    pub const O2_S8_WR_CURRENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3B,
    };
    pub const CATALYST_TEMP_B1S1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3C,
    };
    pub const CATALYST_TEMP_B2S1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3D,
    };
    pub const CATALYST_TEMP_B1S2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3E,
    };
    pub const CATALYST_TEMP_B2S2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x3F,
    };

    // Supported PIDs 41-60
    pub const SUPPORTED_PIDS_41_60: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x40,
    };
    pub const STATUS_DRIVE_CYCLE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x41,
    };
    pub const CONTROL_MODULE_VOLTAGE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x42,
    };
    pub const ABSOLUTE_LOAD: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x43,
    };
    pub const COMMANDED_EQUIV_RATIO: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x44,
    };
    pub const RELATIVE_THROTTLE_POS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x45,
    };
    pub const AMBIANT_AIR_TEMP: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x46,
    };
    pub const THROTTLE_POS_B: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x47,
    };
    pub const THROTTLE_POS_C: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x48,
    };
    pub const ACCELERATOR_POS_D: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x49,
    };
    pub const ACCELERATOR_POS_E: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x4A,
    };
    pub const ACCELERATOR_POS_F: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x4B,
    };
    pub const THROTTLE_ACTUATOR: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x4C,
    };
    pub const RUN_TIME_MIL: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x4D,
    };
    pub const TIME_SINCE_DTC_CLEARED: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x4E,
    };
    pub const MAX_MAF: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x50,
    };
    pub const FUEL_TYPE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x51,
    };
    pub const ETHANOL_PERCENT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x52,
    };
    pub const EVAP_VAPOR_PRESSURE_ABS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x53,
    };
    pub const EVAP_VAPOR_PRESSURE_ALT: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x54,
    };
    pub const SHORT_O2_TRIM_B1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x55,
    };
    pub const LONG_O2_TRIM_B1: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x56,
    };
    pub const SHORT_O2_TRIM_B2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x57,
    };
    pub const LONG_O2_TRIM_B2: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x58,
    };
    pub const FUEL_RAIL_PRESSURE_ABS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x59,
    };
    pub const RELATIVE_ACCEL_POS: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x5A,
    };
    pub const HYBRID_BATTERY_REMAINING: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x5B,
    };
    pub const OIL_TEMP: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x5C,
    };
    pub const FUEL_INJECT_TIMING: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x5D,
    };
    pub const FUEL_RATE: ObdCommand = ObdCommand {
        mode: 0x01,
        pid: 0x5E,
    };
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
