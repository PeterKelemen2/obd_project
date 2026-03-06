use crate::error::ObdError;

/// Known "error" tokens that an ELM327 adapter may return instead of data.
const ADAPTER_ERROR_TOKENS: &[&str] = &[
    "NO DATA",
    "ERROR",
    "UNABLE TO CONNECT",
    "BUS INIT",
    "BUS BUSY",
    "BUS ERROR",
    "CAN ERROR",
    "FB ERROR",
    "DATA ERROR",
    "BUFFER FULL",
    "STOPPED",
    "?",
];

/// Parse a space-separated ASCII hex string returned by the ELM327 adapter
/// into a `Vec<u8>`.
///
/// The function:
/// 1. Strips the ELM327 prompt character (`>`), whitespace, and null bytes.
/// 2. Checks for known adapter-error tokens and returns [`ObdError::AdapterError`].
/// 3. Splits on whitespace and parses each token as a two-digit hex byte.
///
/// # Examples
///
/// ```rust
/// use obd2_reader::response::parse_hex_bytes;
///
/// let bytes = parse_hex_bytes("41 0C 1A F8").unwrap();
/// assert_eq!(bytes, vec![0x41, 0x0C, 0x1A, 0xF8]);
/// ```
pub fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, ObdError> {
    // Normalise: remove prompt chars, null bytes, carriage returns
    let cleaned: String = input
        .chars()
        .filter(|&c| c != '>' && c != '\0' && c != '\r')
        .collect();
    let trimmed = cleaned.trim();

    // Check for well-known error responses (case-insensitive)
    let upper = trimmed.to_uppercase();
    for token in ADAPTER_ERROR_TOKENS {
        if upper.contains(token) {
            return Err(ObdError::AdapterError(trimmed.to_owned()));
        }
    }

    if trimmed.is_empty() {
        return Err(ObdError::Parse("Empty response from adapter".to_owned()));
    }

    trimmed
        .split_whitespace()
        .map(|token| {
            u8::from_str_radix(token, 16).map_err(|_| {
                ObdError::Parse(format!("Invalid hex token: '{token}'"))
            })
        })
        .collect()
}

/// Strip the mode/pid echo bytes from an OBD response and return only the
/// data bytes (byte index 2 onwards).
///
/// ELM327 responses to a Mode 01 query look like:
/// `41 0C 1A F8`
/// where `41` is mode+0x40 echo and `0C` is the PID echo.
/// This helper returns `[0x1A, 0xF8]`.
///
/// # Errors
///
/// Returns [`ObdError::InsufficientData`] if the slice has fewer than
/// `min_data_bytes + 2` bytes.
pub fn extract_data_bytes(raw: &[u8], min_data_bytes: usize) -> Result<&[u8], ObdError> {
    let required = min_data_bytes + 2; // 2 echo bytes
    if raw.len() < required {
        return Err(ObdError::InsufficientData {
            expected: required,
            got: raw.len(),
        });
    }
    Ok(&raw[2..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rpm_response() {
        let bytes = parse_hex_bytes("41 0C 1A F8").unwrap();
        assert_eq!(bytes, vec![0x41, 0x0C, 0x1A, 0xF8]);
    }

    #[test]
    fn parse_with_prompt() {
        let bytes = parse_hex_bytes("41 0D 3C\r\n>").unwrap();
        assert_eq!(bytes, vec![0x41, 0x0D, 0x3C]);
    }

    #[test]
    fn error_no_data() {
        let result = parse_hex_bytes("NO DATA");
        assert!(matches!(result, Err(ObdError::AdapterError(_))));
    }

    #[test]
    fn error_generic() {
        let result = parse_hex_bytes("ERROR");
        assert!(matches!(result, Err(ObdError::AdapterError(_))));
    }

    #[test]
    fn empty_response() {
        let result = parse_hex_bytes("   ");
        assert!(matches!(result, Err(ObdError::Parse(_))));
    }

    #[test]
    fn extract_data_ok() {
        let raw = vec![0x41, 0x0C, 0x1A, 0xF8];
        let data = extract_data_bytes(&raw, 2).unwrap();
        assert_eq!(data, &[0x1A, 0xF8]);
    }

    #[test]
    fn extract_data_insufficient() {
        let raw = vec![0x41, 0x0C];
        let err = extract_data_bytes(&raw, 2).unwrap_err();
        assert!(matches!(err, ObdError::InsufficientData { .. }));
    }
}