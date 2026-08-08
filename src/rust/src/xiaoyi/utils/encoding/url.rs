use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// URL encoding utilities.
///
/// @brief Encode and decode URL strings
/// @since 0.1.0
/// @author Miruamel
pub struct UrlCodec;

impl UrlCodec {
    /// Encode string for safe URL usage.
    ///
    /// @param input Raw input string
    /// @return URL-encoded string
    /// @since 0.1.0
    pub fn encode(&self, input: &str) -> String {
        input
            .bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                    (b as char).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect()
    }

    /// Decode URL-encoded string.
    ///
    /// @param input URL-encoded input
    /// @return Decoded string or error
    /// @since 0.1.0
    pub fn decode(&self, input: &str) -> Result<String, XiaoyiError> {
        let mut output = String::new();
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            if c == '%' {
                let hi = chars.next().ok_or_else(|| {
                    XiaoyiError::new(ErrorKind::Parse, "invalid percent-encoding")
                })?;
                let lo = chars.next().ok_or_else(|| {
                    XiaoyiError::new(ErrorKind::Parse, "invalid percent-encoding")
                })?;
                let byte = u8::from_str_radix(&format!("{hi}{lo}"), 16).map_err(|err| {
                    XiaoyiError::new(ErrorKind::Parse, format!("invalid percent-encoding: {err}"))
                })?;
                output.push(byte as char);
            } else {
                output.push(c);
            }
        }
        Ok(output)
    }
}
