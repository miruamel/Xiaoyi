use serde::de::DeserializeOwned;
use serde_json::Value;

/// Parses a JSON string into a typed value.
///
/// @brief Deserialize JSON string into Rust type
/// @param json JSON string
/// @return Deserialized value or error
/// @since 0.1.0
/// @author Miruamel
/// @see to_json
pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serializes a value to a JSON string.
///
/// @brief Serialize Rust value to JSON string
/// @param value Value to serialize
/// @return JSON string
/// @since 0.1.0
/// @author Miruamel
/// @see from_json
pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Extracts a string field from a JSON object.
///
/// @brief Get string field from JSON object
/// @param obj JSON object
/// @param key Field name
/// @return Field value or None
/// @since 0.1.0
/// @author Miruamel
pub fn get_string_field(obj: &Value, key: &str) -> Option<String> {
    obj.get(key)?.as_str().map(|s| s.to_string())
}

/// Validates that a JSON value is an object.
///
/// @brief Check if JSON value is an object
/// @param value JSON value
/// @return True if value is an object
/// @since 0.1.0
/// @author Miruamel
pub fn is_json_object(value: &Value) -> bool {
    value.is_object()
}
