//! # obd2_reader
//!
//! A Rust library for communicating with ELM327-compatible OBD-II adapters over
//! a serial connection (e.g. `/dev/rfcomm1` on Linux or `COM3` on Windows).
//!
//! ## Quick start
//!
//! ```no_run
//! use obd2_reader::ObdConnection;
//!
//! fn main() -> Result<(), obd2_reader::ObdError> {
//!     let mut obd = ObdConnection::connect("/dev/rfcomm1")?;
//!     obd.initialize()?;
//!
//!     let rpm   = obd.query_engine_rpm()?;
//!     let speed = obd.query_vehicle_speed()?;
//!     let temp  = obd.query_coolant_temp()?;
//!
//!     println!("RPM:              {rpm:.0}");
//!     println!("Speed:            {speed} km/h");
//!     println!("Coolant temp:     {temp} °C");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Module overview
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`connection`] | Serial port wrapper ([`SerialConnection`]) and high-level API ([`ObdConnection`]) |
//! | [`command`]    | [`ObdCommand`] struct and well-known PID constants |
//! | [`response`]   | Hex response parsing ([`response::parse_hex_bytes`]) |
//! | [`pid`]        | PID decoding functions (`decode_engine_rpm`, etc.) |
//! | [`error`]      | [`ObdError`] custom error enum |

pub mod command;
pub mod connection;
pub mod error;
pub mod pid;
pub mod response;

// ---------------------------------------------------------------------------
// Convenience re-exports from the crate root
// ---------------------------------------------------------------------------

/// The primary high-level entry point — re-exported for ergonomic import.
pub use connection::ObdConnection;

/// Low-level serial connection — re-exported for users who need it directly.
pub use connection::SerialConnection;

/// The OBD command builder struct.
pub use command::ObdCommand;

/// The crate-wide error type.
pub use error::ObdError;