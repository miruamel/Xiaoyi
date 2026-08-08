use std::path::{Path, PathBuf};

/// Reads a file to a string.
///
/// @brief Read entire file content as string
/// @param path File path
/// @return File content or error
/// @since 0.1.0
/// @author Miruamel
pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Writes a string to a file.
///
/// @brief Write string content to file
/// @param path File path
/// @param content Content to write
/// @since 0.1.0
/// @author Miruamel
pub fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(path, content)
}

/// Returns the canonical absolute path.
///
/// @brief Resolve canonical absolute path
/// @param path Input path
/// @return Canonical path or error
/// @since 0.1.0
/// @author Miruamel
pub fn canonicalize(path: &Path) -> Result<PathBuf, std::io::Error> {
    std::fs::canonicalize(path)
}
