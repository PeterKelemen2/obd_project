/// Custom error type for all OBD-II operations.
///
/// All fallible functions in this library return `Result<T, ObdError>`.
#[derive(Debug, thiserror::Error)]
pub enum ObdError {
    /// Wraps errors from the `serialport` crate (open failures, I/O errors, etc.)
    #[error("Serial port error: {0}")]
    Serial(#[from] serialport::Error),

    /// An I/O error occurred while reading from or writing to the serial port.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The adapter returned no data within the configured timeout.
    #[error("Read timeout: no data received from adapter")]
    Timeout,

    /// The adapter returned a response that could not be parsed.
    #[error("Parse error: {0}")]
    Parse(String),

    /// The adapter returned an error token (e.g. "NO DATA", "ERROR", "UNABLE TO CONNECT").
    #[error("Adapter error response: {0}")]
    AdapterError(String),

    /// The OBD response did not contain enough bytes for the requested PID.
    #[error("Insufficient data bytes: expected {expected}, got {got}")]
    InsufficientData { expected: usize, got: usize },

    /// The ELM327 initialisation sequence failed.
    #[error("Initialisation failed: {0}")]
    InitFailed(String),
}