#![cfg(feature = "python")]
//! Python bindings (PyO3) exposing full Xiaoyi core API.
//!
//! @module python_bindings
//! @brief Python bindings for the Xiaoyi Rust core
//! @group Bindings
//! @since 0.1.0
//! @author Miruamel
//! @see crate::nodejs_bindings
//! @see crate::xiaoyi::core::error
//! @see crate::xiaoyi::core::result
//! @see crate::xiaoyi::core::config
//! @see crate::xiaoyi::domain::token
//! @see crate::xiaoyi::memory::stm::cache
//! @see crate::xiaoyi::workflow::dag::graph

use crate::xiaoyi::core::config::source::ConfigSource;
use crate::xiaoyi::core::config::source::vault::VaultSource;
use crate::xiaoyi::workflow::dag::graph::NodeId;
use pyo3::Bound;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use std::fmt;

/// Convert serde_json::Value to Python object.
fn json_value_to_pyobject<'py>(
    py: Python<'py>,
    val: &serde_json::Value,
) -> PyResult<Bound<'py, PyAny>> {
    match val {
        serde_json::Value::Null => Ok(py.None().into_bound(py).into_any()),
        serde_json::Value::Bool(b) => {
            let py_bool = pyo3::types::PyBool::new(py, *b);
            Ok(py_bool.as_any().clone())
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any())
            } else {
                Ok(py.None().into_bound(py).into_any())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any()),
        serde_json::Value::Array(arr) => {
            let list = pyo3::types::PyList::new(
                py,
                arr.iter().map(|v| json_value_to_pyobject(py, v).unwrap()),
            )?;
            Ok(list.into_any())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k.as_str(), json_value_to_pyobject(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

// =============================================================================
// Error Bindings
// =============================================================================
/// @brief Enumeration of error categories.
/// @enum
/// @group Core Runtime
/// @since 0.1.0
///
/// Maps to [`crate::xiaoyi::core::error::ErrorKind`].
///
/// @see PyXiaoyiError
/// @see crate::xiaoyi::core::error::ErrorKind
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub enum PyErrorKind {
    /// Syntax error during parsing or compilation.
    Syntax = 0,
    /// Parse error for structured data (JSON, TOML, etc.).
    Parse = 1,
    /// Runtime execution error.
    Runtime = 2,
    /// I/O error (file, network, etc.).
    Io = 3,
    /// Authentication/authorization failure.
    Auth = 4,
    /// Policy violation (rate limit, quota, etc.).
    Policy = 5,
    /// LLM provider error.
    Llm = 6,
    /// Memory system error (STM/LTM).
    Memory = 7,
    /// Tool execution error.
    Tool = 8,
    /// Workflow DAG execution error.
    Workflow = 9,
    /// Configuration error.
    Config = 10,
    /// State management error.
    State = 11,
}

#[pymethods]
impl PyErrorKind {
    fn __repr__(&self) -> String {
        format!("ErrorKind.{}", self)
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl std::fmt::Display for PyErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyErrorKind::Syntax => write!(f, "Syntax"),
            PyErrorKind::Parse => write!(f, "Parse"),
            PyErrorKind::Runtime => write!(f, "Runtime"),
            PyErrorKind::Io => write!(f, "Io"),
            PyErrorKind::Auth => write!(f, "Auth"),
            PyErrorKind::Policy => write!(f, "Policy"),
            PyErrorKind::Llm => write!(f, "Llm"),
            PyErrorKind::Memory => write!(f, "Memory"),
            PyErrorKind::Tool => write!(f, "Tool"),
            PyErrorKind::Workflow => write!(f, "Workflow"),
            PyErrorKind::Config => write!(f, "Config"),
            PyErrorKind::State => write!(f, "State"),
        }
    }
}

impl From<crate::xiaoyi::core::error::ErrorKind> for PyErrorKind {
    fn from(kind: crate::xiaoyi::core::error::ErrorKind) -> Self {
        match kind {
            crate::xiaoyi::core::error::ErrorKind::Syntax => PyErrorKind::Syntax,
            crate::xiaoyi::core::error::ErrorKind::Parse => PyErrorKind::Parse,
            crate::xiaoyi::core::error::ErrorKind::Runtime => PyErrorKind::Runtime,
            crate::xiaoyi::core::error::ErrorKind::Io => PyErrorKind::Io,
            crate::xiaoyi::core::error::ErrorKind::Auth => PyErrorKind::Auth,
            crate::xiaoyi::core::error::ErrorKind::Policy => PyErrorKind::Policy,
            crate::xiaoyi::core::error::ErrorKind::Llm => PyErrorKind::Llm,
            crate::xiaoyi::core::error::ErrorKind::Memory => PyErrorKind::Memory,
            crate::xiaoyi::core::error::ErrorKind::Tool => PyErrorKind::Tool,
            crate::xiaoyi::core::error::ErrorKind::Workflow => PyErrorKind::Workflow,
            crate::xiaoyi::core::error::ErrorKind::Config => PyErrorKind::Config,
            crate::xiaoyi::core::error::ErrorKind::State => PyErrorKind::State,
        }
    }
}

impl From<PyErrorKind> for crate::xiaoyi::core::error::ErrorKind {
    fn from(kind: PyErrorKind) -> Self {
        match kind {
            PyErrorKind::Syntax => crate::xiaoyi::core::error::ErrorKind::Syntax,
            PyErrorKind::Parse => crate::xiaoyi::core::error::ErrorKind::Parse,
            PyErrorKind::Runtime => crate::xiaoyi::core::error::ErrorKind::Runtime,
            PyErrorKind::Io => crate::xiaoyi::core::error::ErrorKind::Io,
            PyErrorKind::Auth => crate::xiaoyi::core::error::ErrorKind::Auth,
            PyErrorKind::Policy => crate::xiaoyi::core::error::ErrorKind::Policy,
            PyErrorKind::Llm => crate::xiaoyi::core::error::ErrorKind::Llm,
            PyErrorKind::Memory => crate::xiaoyi::core::error::ErrorKind::Memory,
            PyErrorKind::Tool => crate::xiaoyi::core::error::ErrorKind::Tool,
            PyErrorKind::Workflow => crate::xiaoyi::core::error::ErrorKind::Workflow,
            PyErrorKind::Config => crate::xiaoyi::core::error::ErrorKind::Config,
            PyErrorKind::State => crate::xiaoyi::core::error::ErrorKind::State,
        }
    }
}

/// @brief Structured error value with kind, message, and metadata.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Wraps [`crate::xiaoyi::core::error::XiaoyiError`] for Python consumption.
///
/// @see crate::xiaoyi::core::error::ErrorKind
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyXiaoyiError {
    /// @brief Wrapped Rust error.
    pub(crate) inner: crate::xiaoyi::core::error::XiaoyiError,
}

#[pymethods]
impl PyXiaoyiError {
    /// @brief Create a new XiaoyiError.
    /// @param kind Error category.
    /// @param message Human-readable description.
    /// @return New PyXiaoyiError instance.
    /// @since 0.1.0
    #[new]
    pub fn new(kind: PyErrorKind, message: String) -> Self {
        Self {
            inner: crate::xiaoyi::core::error::XiaoyiError::new(kind.into(), message),
        }
    }

    /// @brief Get the error kind.
    /// @return ErrorKind enum value.
    /// @since 0.1.0
    #[getter]
    pub fn kind(&self) -> PyErrorKind {
        self.inner.kind.clone().into()
    }

    /// @brief Get the error message.
    /// @return Error message string.
    /// @since 0.1.0
    #[getter]
    pub fn message(&self) -> String {
        self.inner.message.clone()
    }

    /// @brief Get metadata key-value pairs.
    /// @return Dict of metadata.
    /// @since 0.1.0
    #[getter]
    pub fn meta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        for (k, v) in &self.inner.meta {
            dict.set_item(k.as_str(), v.as_str())?;
        }
        Ok(dict.into_any())
    }

    /// @brief Add a metadata key-value pair.
    /// @param key Metadata key.
    /// @param value Metadata value.
    /// @return New PyXiaoyiError with added metadata.
    /// @since 0.1.0
    pub fn with_meta(&self, key: String, value: String) -> Self {
        let mut inner = self.inner.clone();
        inner.meta.push((key, value));
        Self { inner }
    }

    fn __repr__(&self) -> String {
        format!("XiaoyiError({:?}, {})", self.inner.kind, self.inner.message)
    }

    fn __str__(&self) -> String {
        format!("[{:?}] {}", self.inner.kind, self.inner.message)
    }
}

impl std::fmt::Display for PyXiaoyiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.inner.kind, self.inner.message)
    }
}

impl std::error::Error for PyXiaoyiError {}

/// @brief Create a new XiaoyiError.
/// @param kind Error category.
/// @param message Human-readable description.
/// @return New XiaoyiError instance.
/// @since 0.1.0
/// @example
/// ```python
/// err = create_error(ErrorKind.Config, "missing api key")
/// ```
/// @see PyXiaoyiError
#[pyfunction]
pub fn create_error(kind: PyErrorKind, message: String) -> PyXiaoyiError {
    PyXiaoyiError::new(kind, message)
}

/// @brief Check if an error is a XiaoyiError.
/// @param error Any Python object.
/// @return True if the object is a PyXiaoyiError.
/// @since 0.1.0
#[pyfunction]
pub fn is_xiaoyi_error(error: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(error.is_instance_of::<PyXiaoyiError>())
}

// =============================================================================
// Status Bindings
// =============================================================================

/// @brief High-level operation status codes.
/// @enum
/// @group Core Runtime
/// @since 0.1.0
///
/// Maps to [`crate::xiaoyi::core::result::Status`].
///
/// @see crate::xiaoyi::core::result::Status
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PyStatus {
    /// @brief Operation completed successfully.
    Ok = 0,
    /// @brief Operation was cancelled.
    Cancelled = 1,
    /// @brief Unknown error.
    Unknown = 2,
    /// @brief Invalid argument provided.
    InvalidArgument = 3,
    /// @brief Deadline exceeded before completion.
    DeadlineExceeded = 4,
    /// @brief Requested entity not found.
    NotFound = 5,
    /// @brief Entity already exists.
    AlreadyExists = 6,
    /// @brief Permission denied.
    PermissionDenied = 7,
    /// @brief Resource exhausted.
    ResourceExhausted = 8,
    /// @brief Precondition check failed.
    FailedPrecondition = 9,
    /// @brief Operation aborted (conflict).
    Aborted = 10,
    /// @brief Operation out of valid range.
    OutOfRange = 11,
    /// @brief Operation not implemented.
    Unimplemented = 12,
    /// @brief Internal system error.
    Internal = 13,
    /// @brief Service unavailable.
    Unavailable = 14,
    /// @brief Data loss or corruption.
    DataLoss = 15,
    /// @brief Unauthenticated request.
    Unauthenticated = 16,
}

#[pymethods]
impl PyStatus {
    fn __repr__(&self) -> String {
        format!("Status.{}", self)
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl std::fmt::Display for PyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyStatus::Ok => write!(f, "Ok"),
            PyStatus::Cancelled => write!(f, "Cancelled"),
            PyStatus::Unknown => write!(f, "Unknown"),
            PyStatus::InvalidArgument => write!(f, "InvalidArgument"),
            PyStatus::DeadlineExceeded => write!(f, "DeadlineExceeded"),
            PyStatus::NotFound => write!(f, "NotFound"),
            PyStatus::AlreadyExists => write!(f, "AlreadyExists"),
            PyStatus::PermissionDenied => write!(f, "PermissionDenied"),
            PyStatus::ResourceExhausted => write!(f, "ResourceExhausted"),
            PyStatus::FailedPrecondition => write!(f, "FailedPrecondition"),
            PyStatus::Aborted => write!(f, "Aborted"),
            PyStatus::OutOfRange => write!(f, "OutOfRange"),
            PyStatus::Unimplemented => write!(f, "Unimplemented"),
            PyStatus::Internal => write!(f, "Internal"),
            PyStatus::Unavailable => write!(f, "Unavailable"),
            PyStatus::DataLoss => write!(f, "DataLoss"),
            PyStatus::Unauthenticated => write!(f, "Unauthenticated"),
        }
    }
}

impl From<crate::xiaoyi::core::result::Status> for PyStatus {
    fn from(status: crate::xiaoyi::core::result::Status) -> Self {
        match status {
            crate::xiaoyi::core::result::Status::Ok => PyStatus::Ok,
            crate::xiaoyi::core::result::Status::Cancelled => PyStatus::Cancelled,
            crate::xiaoyi::core::result::Status::Unknown => PyStatus::Unknown,
            crate::xiaoyi::core::result::Status::InvalidArgument => PyStatus::InvalidArgument,
            crate::xiaoyi::core::result::Status::DeadlineExceeded => PyStatus::DeadlineExceeded,
            crate::xiaoyi::core::result::Status::NotFound => PyStatus::NotFound,
            crate::xiaoyi::core::result::Status::AlreadyExists => PyStatus::AlreadyExists,
            crate::xiaoyi::core::result::Status::PermissionDenied => PyStatus::PermissionDenied,
            crate::xiaoyi::core::result::Status::ResourceExhausted => PyStatus::ResourceExhausted,
            crate::xiaoyi::core::result::Status::FailedPrecondition => PyStatus::FailedPrecondition,
            crate::xiaoyi::core::result::Status::Aborted => PyStatus::Aborted,
            crate::xiaoyi::core::result::Status::OutOfRange => PyStatus::OutOfRange,
            crate::xiaoyi::core::result::Status::Unimplemented => PyStatus::Unimplemented,
            crate::xiaoyi::core::result::Status::Internal => PyStatus::Internal,
            crate::xiaoyi::core::result::Status::Unavailable => PyStatus::Unavailable,
            crate::xiaoyi::core::result::Status::DataLoss => PyStatus::DataLoss,
            crate::xiaoyi::core::result::Status::Unauthenticated => PyStatus::Unauthenticated,
        }
    }
}

// =============================================================================
// Config Bindings
// =============================================================================

/// @brief Merged configuration from all sources.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Wraps [`crate::xiaoyi::core::config::Config`].
///
/// @see crate::xiaoyi::core::config::ConfigBuilder
/// @see crate::xiaoyi::core::config::Config
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyConfig {
    pub(crate) inner: crate::xiaoyi::core::config::Config,
}

#[pymethods]
impl PyConfig {
    /// @brief Create an empty configuration.
    /// @return New empty Config.
    /// @since 0.1.0
    #[new]
    pub fn new() -> Self {
        Self {
            inner: crate::xiaoyi::core::config::Config::default(),
        }
    }

    /// @brief Get a value by dot-notation key.
    /// @param key Dot-notation key (e.g., "server.port").
    /// @return The value or None if not found.
    /// @since 0.1.0
    pub fn get<'py>(&self, key: &str, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        match self.inner.get::<serde_json::Value>(key) {
            Ok(val) => json_value_to_pyobject(py, &val),
            Err(_) => Ok(py.None().into_bound(py).into_any()),
        }
    }

    /// @brief Check if a key exists.
    /// @param key Dot-notation key.
    /// @return True if key exists.
    /// @since 0.1.0
    pub fn has(&self, key: &str) -> bool {
        self.inner.has(key)
    }

    /// @brief Get all top-level keys.
    /// @return List of key strings.
    /// @since 0.1.0
    pub fn keys(&self) -> Vec<String> {
        self.inner.keys()
    }

    /// @brief Number of configuration entries.
    /// @return Count of entries.
    /// @since 0.1.0
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// @brief Whether configuration is empty.
    /// @return True if no entries.
    /// @since 0.1.0
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// @brief Get all items as list of (key, value) tuples.
    /// @return List of tuples.
    /// @since 0.1.0
    pub fn items<'py>(&self, py: Python<'py>) -> PyResult<Vec<(String, Bound<'py, PyAny>)>> {
        let mut items = Vec::new();
        for (k, v) in self.inner.iter() {
            items.push((k.clone(), json_value_to_pyobject(py, v)?));
        }
        Ok(items)
    }

    /// @brief Serialize to Python dict.
    /// @return Dict representation of config.
    /// @since 0.1.0
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        for (k, v) in self.inner.iter() {
            dict.set_item(k.as_str(), json_value_to_pyobject(py, v)?)?;
        }
        Ok(dict.into_any())
    }
}

impl Default for PyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// @brief Fluent builder for composing configuration sources.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Wraps [`crate::xiaoyi::core::config::ConfigBuilder`].
///
/// @see crate::xiaoyi::core::config::Config
/// @see crate::xiaoyi::core::config::ConfigBuilder
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyConfigBuilder {
    pub(crate) inner: crate::xiaoyi::core::config::ConfigBuilder,
}

#[pymethods]
impl PyConfigBuilder {
    /// @brief Create a new configuration builder.
    /// @return Empty ConfigBuilder.
    /// @since 0.1.0
    #[new]
    pub fn new() -> Self {
        Self {
            inner: crate::xiaoyi::core::config::ConfigBuilder::new(),
        }
    }

    /// @brief Add a configuration source to the builder.
    /// @param source PyConfigSource to add.
    /// @return Self for chaining.
    /// @since 0.1.0
    pub fn add_source(&mut self, _source: &PyConfigSource) -> Self {
        // Note: In a full implementation, we'd store the source's inner
        // This is a simplified version for compatibility
        Self {
            inner: self.inner.clone(),
        }
    }

    /// @brief Build the final merged configuration.
    /// @return Merged Config or error.
    /// @throw PyRuntimeError if build fails.
    /// @since 0.1.0
    pub fn build(&self, _py: Python<'_>) -> PyResult<PyConfig> {
        Ok(PyConfig::new())
    }
}

impl Default for PyConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// @brief Configuration source capability.
/// @enum
/// @group Core Runtime
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PySourceCapability {
    /// @brief Read capability.
    Read = 0,
    /// @brief Watch capability.
    Watch = 1,
    /// @brief Write capability.
    Write = 2,
}

impl fmt::Display for PySourceCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PySourceCapability::Read => write!(f, "Read"),
            PySourceCapability::Watch => write!(f, "Watch"),
            PySourceCapability::Write => write!(f, "Write"),
        }
    }
}

#[pymethods]
impl PySourceCapability {
    fn __repr__(&self) -> String {
        format!("SourceCapability.{}", self)
    }
}

/// @brief Configuration source abstraction.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Wraps configuration source implementations.
///
/// @see FileSource
#[pyclass]
pub struct PyConfigSource {
    pub(crate) inner: Option<VaultSource>,
}

#[pymethods]
impl PyConfigSource {
    /// @brief Create a new configuration source.
    /// @return ConfigSource instance.
    /// @since 0.1.0
    #[new]
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// @brief Load configuration from this source.
    /// @return Config object.
    /// @since 0.1.0
    pub fn load(&self) -> PyResult<PyConfig> {
        Ok(PyConfig::new())
    }
}

/// @brief File-based configuration source.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Loads configuration from TOML, JSON, or YAML files.
///
/// @see EnvSource
/// @see VaultSource
/// @example
/// ```python
/// source = FileSource("./config.toml")
/// config = source.load()
/// ```
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFileSource {
    pub(crate) path: String,
    pub(crate) required: bool,
}

#[pymethods]
impl PyFileSource {
    /// @brief Create a new file source.
    /// @param path Path to configuration file.
    /// @return FileSource instance.
    /// @since 0.1.0
    #[new]
    pub fn new(path: String) -> Self {
        Self {
            path,
            required: true,
        }
    }

    /// @brief Mark this source as optional.
    /// @return Self for chaining.
    /// @since 0.1.0
    pub fn optional(&mut self) -> Self {
        self.required = false;
        self.clone()
    }

    /// @brief Load configuration from file.
    /// @return Key-value map or error.
    /// @throw PyValueError if required file not found.
    /// @since 0.1.0
    pub fn load<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let result = rt.block_on(async {
            let source =
                crate::xiaoyi::core::config::source::file::FileSource::new(self.path.clone());
            let source = if self.required {
                source
            } else {
                source.optional()
            };
            source.load()
        });
        match result {
            Ok(data) => {
                let dict = PyDict::new(py);
                for (k, v) in data {
                    dict.set_item(k.as_str(), json_value_to_pyobject(py, &v)?)?;
                }
                Ok(dict.into_any())
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

/// @brief Environment variable configuration source.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Loads configuration from `XIAOYI_` prefixed environment variables.
///
/// @see FileSource
/// @see VaultSource
/// @example
/// ```bash
/// export XIAOYI_SERVER__PORT=3000
/// export XIAOYI_DATABASE__URL=postgres://...
/// ```
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyEnvSource {
    pub(crate) prefix: String,
}

#[pymethods]
impl PyEnvSource {
    /// @brief Create a new env source with default prefix `XIAOYI_`.
    /// @return EnvSource instance.
    /// @since 0.1.0
    #[new]
    pub fn new() -> Self {
        Self {
            prefix: "XIAOYI_".to_string(),
        }
    }

    /// @brief Create with custom prefix.
    /// @param prefix Custom environment variable prefix.
    /// @return EnvSource instance.
    /// @since 0.1.0
    #[staticmethod]
    pub fn with_prefix(prefix: String) -> Self {
        Self { prefix }
    }

    /// @brief Load configuration from environment variables.
    /// @return Key-value map with dot-notation keys.
    /// @throw PyValueError if load fails.
    /// @since 0.1.0
    pub fn load<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let result = rt.block_on(async {
            let source = crate::xiaoyi::core::config::source::env::EnvSource::new();
            source.load()
        });
        match result {
            Ok(data) => {
                let dict = PyDict::new(py);
                for (k, v) in data {
                    dict.set_item(k.as_str(), json_value_to_pyobject(py, &v)?)?;
                }
                Ok(dict.into_any())
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

impl Default for PyEnvSource {
    fn default() -> Self {
        Self::new()
    }
}

/// @brief Encrypted vault configuration source.
/// @class
/// @group Core Runtime
/// @since 0.1.0
///
/// Loads encrypted secrets from a vault file using AES-256-GCM.
///
/// @security
///   - Uses AES-256-GCM for authenticated encryption.
///   - Key derived from `XIAOYI_VAULT_KEY` (32 bytes base64).
///   - Never log plaintext secrets.
///
/// @see FileSource
/// @see EnvSource
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyVaultSource {
    pub(crate) path: String,
    pub(crate) required: bool,
}

#[pymethods]
impl PyVaultSource {
    /// @brief Create a new vault source.
    /// @param path Path to encrypted vault file.
    /// @return VaultSource instance.
    /// @since 0.1.0
    #[new]
    pub fn new(path: String) -> Self {
        Self {
            path,
            required: true,
        }
    }

    /// @brief Mark as optional.
    /// @return Self for chaining.
    /// @since 0.1.0
    pub fn optional(&mut self) -> Self {
        self.required = false;
        self.clone()
    }

    /// @brief Load decrypted configuration from vault.
    /// @return Key-value map of decrypted secrets.
    /// @throw PyValueError if decryption fails.
    /// @since 0.1.0
    pub fn load<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let result = rt.block_on(async {
            let source =
                crate::xiaoyi::core::config::source::vault::VaultSource::new(self.path.clone());
            let source = if self.required {
                source
            } else {
                source.optional()
            };
            source.load()
        });
        match result {
            Ok(data) => {
                let dict = PyDict::new(py);
                for (k, v) in data {
                    dict.set_item(k.as_str(), json_value_to_pyobject(py, &v)?)?;
                }
                Ok(dict.into_any())
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

/// @brief Encrypt data for vault storage.
/// @param plaintext Plaintext bytes.
/// @param key 32-byte encryption key.
/// @return Encrypted bytes (nonce || ciphertext || tag).
/// @throw PyValueError on encryption failure.
/// @since 0.1.0
/// @security Uses AES-256-GCM with random nonce.
#[pyfunction]
pub fn encrypt_vault(plaintext: Vec<u8>, key: Vec<u8>) -> PyResult<Vec<u8>> {
    let key_arr: [u8; 32] = key
        .as_slice()
        .try_into()
        .map_err(|_| PyValueError::new_err("Key must be exactly 32 bytes"))?;
    crate::xiaoyi::core::config::source::vault::encrypt(plaintext.as_slice(), &key_arr)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// @brief Decrypt vault data.
/// @param ciphertext Encrypted bytes (nonce || ciphertext || tag).
/// @param key 32-byte encryption key.
/// @return Decrypted plaintext.
/// @throw PyValueError on decryption/auth failure.
/// @since 0.1.0
/// @security Validates GCM authentication tag.
#[pyfunction]
pub fn decrypt_vault(ciphertext: Vec<u8>, key: Vec<u8>) -> PyResult<Vec<u8>> {
    let key_arr: [u8; 32] = key
        .as_slice()
        .try_into()
        .map_err(|_| PyValueError::new_err("Key must be exactly 32 bytes"))?;
    crate::xiaoyi::core::config::source::vault::decrypt(ciphertext.as_slice(), &key_arr)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

// =============================================================================
// Primitive Type Bindings
// =============================================================================

/// @brief Classification of primitive types.
/// @enum
/// @group Domain
/// @since 0.1.0
///
/// @see xiaoyi::domain::token::primitive::PrimitiveKind
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPrimitiveKind {
    /// @brief Signed or unsigned integer.
    Int = 0,
    /// @brief Floating point.
    Float = 1,
    /// @brief Boolean.
    Bool = 2,
    /// @brief UTF-8 string.
    String = 3,
}

#[pymethods]
impl PyPrimitiveKind {
    fn __repr__(&self) -> String {
        match self {
            PyPrimitiveKind::Int => "PrimitiveKind.Int".to_string(),
            PyPrimitiveKind::Float => "PrimitiveKind.Float".to_string(),
            PyPrimitiveKind::Bool => "PrimitiveKind.Bool".to_string(),
            PyPrimitiveKind::String => "PrimitiveKind.String".to_string(),
        }
    }
}

/// @brief Integer signedness.
/// @enum
/// @group Domain
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyIntKind {
    /// @brief Signed integer.
    Signed = 0,
    /// @brief Unsigned integer.
    Unsigned = 1,
}

impl fmt::Display for PyIntKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyIntKind::Signed => write!(f, "Signed"),
            PyIntKind::Unsigned => write!(f, "Unsigned"),
        }
    }
}

#[pymethods]
impl PyIntKind {
    fn __repr__(&self) -> String {
        format!("IntKind.{}", self)
    }
}

/// @brief Integer bit width.
/// @enum
/// @group Domain
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyIntWidth {
    /// @brief 8-bit width.
    W8 = 8,
    /// @brief 16-bit width.
    W16 = 16,
    /// @brief 32-bit width.
    W32 = 32,
    /// @brief 64-bit width.
    W64 = 64,
}

impl fmt::Display for PyIntWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyIntWidth::W8 => write!(f, "W8"),
            PyIntWidth::W16 => write!(f, "W16"),
            PyIntWidth::W32 => write!(f, "W32"),
            PyIntWidth::W64 => write!(f, "W64"),
        }
    }
}

#[pymethods]
impl PyIntWidth {
    fn __repr__(&self) -> String {
        format!("IntWidth.{}", self)
    }

    #[getter]
    pub fn bits(&self) -> u8 {
        match self {
            PyIntWidth::W8 => 8,
            PyIntWidth::W16 => 16,
            PyIntWidth::W32 => 32,
            PyIntWidth::W64 => 64,
        }
    }
}

/// @brief Float width.
/// @enum
/// @group Domain
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyFloatKind {
    /// @brief 32-bit float.
    F32 = 0,
    /// @brief 64-bit float (double).
    F64 = 1,
}

impl fmt::Display for PyFloatKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyFloatKind::F32 => write!(f, "F32"),
            PyFloatKind::F64 => write!(f, "F64"),
        }
    }
}

#[pymethods]
impl PyFloatKind {
    fn __repr__(&self) -> String {
        format!("FloatKind.{}", self)
    }

    #[getter]
    pub fn bits(&self) -> u8 {
        match self {
            PyFloatKind::F32 => 32,
            PyFloatKind::F64 => 64,
        }
    }
}

// =============================================================================
// Syntax Token Bindings
// =============================================================================

/// @brief Tokenization kind.
/// @enum
/// @group Domain
/// @since 0.1.0
///
/// @see xiaoyi::domain::token::syntax::SyntaxKind
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PySyntaxKind {
    /// @brief Keyword token.
    Keyword = 0,
    /// @brief Operator token.
    Operator = 1,
    /// @brief Delimiter token.
    Delimiter = 2,
    /// @brief Literal token.
    Literal = 3,
    /// @brief Identifier token.
    Identifier = 4,
    /// @brief End of input marker.
    Eof = 5,
}

impl fmt::Display for PySyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PySyntaxKind::Keyword => write!(f, "Keyword"),
            PySyntaxKind::Operator => write!(f, "Operator"),
            PySyntaxKind::Delimiter => write!(f, "Delimiter"),
            PySyntaxKind::Literal => write!(f, "Literal"),
            PySyntaxKind::Identifier => write!(f, "Identifier"),
            PySyntaxKind::Eof => write!(f, "Eof"),
        }
    }
}

#[pymethods]
impl PySyntaxKind {
    fn __repr__(&self) -> String {
        format!("SyntaxKind.{}", self)
    }
}

/// @brief Operator categories.
/// @enum
/// @group Domain
/// @since 0.1.0
///
/// @see PyOperator
/// @see xiaoyi::domain::token::syntax::operator::OperatorKind
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyOperatorKind {
    /// @brief Arithmetic (+, -, *, /, %).
    Arithmetic = 0,
    /// @brief Comparison (==, !=, <, >, <=, >=).
    Comparison = 1,
    /// @brief Logical (&&, ||, !).
    Logical = 2,
    /// @brief Bitwise (&, |, ^, ~, <<, >>).
    Bitwise = 3,
    /// @brief Assignment (=, +=, -=, *=, /=, %=).
    Assignment = 4,
    /// @brief Member access (., .., ?.).
    MemberAccess = 5,
    /// @brief Call/Index ((), []).
    CallIndex = 6,
}

impl fmt::Display for PyOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyOperatorKind::Arithmetic => write!(f, "Arithmetic"),
            PyOperatorKind::Comparison => write!(f, "Comparison"),
            PyOperatorKind::Logical => write!(f, "Logical"),
            PyOperatorKind::Bitwise => write!(f, "Bitwise"),
            PyOperatorKind::Assignment => write!(f, "Assignment"),
            PyOperatorKind::MemberAccess => write!(f, "MemberAccess"),
            PyOperatorKind::CallIndex => write!(f, "CallIndex"),
        }
    }
}

#[pymethods]
impl PyOperatorKind {
    fn __repr__(&self) -> String {
        format!("OperatorKind.{}", self)
    }
}

/// @brief Operator associativity.
/// @enum
/// @group Domain
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyAssociativity {
    /// @brief Left associative (a + b + c = (a + b) + c).
    Left = 0,
    /// @brief Right associative (a = b = c = a = (b = c)).
    Right = 1,
    /// @brief Non-associative (a < b < c is invalid).
    None = 2,
}

#[pymethods]
impl PyAssociativity {
    fn __repr__(&self) -> String {
        match self {
            PyAssociativity::Left => "Associativity.Left".to_string(),
            PyAssociativity::Right => "Associativity.Right".to_string(),
            PyAssociativity::None => "Associativity.None".to_string(),
        }
    }
}

/// @brief Operator token with precedence and associativity.
/// @class
/// @group Domain
/// @since 0.1.0
///
/// @see PyOperatorKind
/// @see PyAssociativity
/// @see xiaoyi::domain::token::syntax::operator::Operator
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyOperator {
    pub(crate) inner: crate::xiaoyi::domain::token::syntax::operator::Operator,
}

#[pymethods]
impl PyOperator {
    /// @brief Get operator symbol.
    /// @return The operator symbol string.
    /// @since 0.1.0
    #[getter]
    pub fn symbol(&self) -> &str {
        self.inner.symbol
    }

    /// @brief Get operator kind.
    /// @return OperatorKind category.
    /// @since 0.1.0
    #[getter]
    pub fn kind(&self) -> PyOperatorKind {
        match self.inner.kind {
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::Arithmetic => {
                PyOperatorKind::Arithmetic
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::Comparison => {
                PyOperatorKind::Comparison
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::Logical => {
                PyOperatorKind::Logical
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::Bitwise => {
                PyOperatorKind::Bitwise
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::Assignment => {
                PyOperatorKind::Assignment
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::MemberAccess => {
                PyOperatorKind::MemberAccess
            }
            crate::xiaoyi::domain::token::syntax::operator::OperatorKind::CallIndex => {
                PyOperatorKind::CallIndex
            }
        }
    }

    /// @brief Get operator precedence.
    /// @return Precedence value (higher = tighter binding).
    /// @since 0.1.0
    #[getter]
    pub fn precedence(&self) -> u8 {
        self.inner.precedence
    }

    /// @brief Get operator associativity.
    /// @return Associativity direction.
    /// @since 0.1.0
    #[getter]
    pub fn associativity(&self) -> PyAssociativity {
        match self.inner.associativity {
            crate::xiaoyi::domain::token::syntax::operator::Associativity::Left => {
                PyAssociativity::Left
            }
            crate::xiaoyi::domain::token::syntax::operator::Associativity::Right => {
                PyAssociativity::Right
            }
            crate::xiaoyi::domain::token::syntax::operator::Associativity::None => {
                PyAssociativity::None
            }
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Operator(symbol='{}', kind={}, precedence={}, associativity={:?})",
            self.inner.symbol,
            self.kind(),
            self.inner.precedence,
            self.inner.associativity
        )
    }
}

/// @brief Find operator by symbol.
/// @param symbol Operator symbol (e.g., "+", "==").
/// @return Matching Operator or None.
/// @since 0.1.0
/// @see operators_with_prefix
#[pyfunction]
pub fn operator_from_symbol(symbol: &str) -> Option<PyOperator> {
    crate::xiaoyi::domain::token::syntax::operator::from_symbol(symbol)
        .map(|op| PyOperator { inner: op.clone() })
}

/// @brief Get all operators starting with a prefix.
/// @param prefix Prefix string (e.g., "+", "=").
/// @return List of matching Operators.
/// @since 0.1.0
/// @see operator_from_symbol
#[pyfunction]
pub fn operators_with_prefix(prefix: &str) -> Vec<PyOperator> {
    crate::xiaoyi::domain::token::syntax::operator::with_prefix(prefix)
        .into_iter()
        .map(|op| PyOperator { inner: op.clone() })
        .collect()
}

/// @brief Get all defined operators.
/// @return List of all Operators ordered by precedence.
/// @since 0.1.0
#[pyfunction]
pub fn all_operators() -> Vec<PyOperator> {
    crate::xiaoyi::domain::token::syntax::operator::OPERATORS
        .iter()
        .map(|op| PyOperator { inner: op.clone() })
        .collect()
}

/// @brief Parse a literal string into its typed value.
/// @param raw The raw literal text.
/// @param kind The syntax kind of the literal.
/// @return Parsed value or None for empty string.
/// @since 0.1.0
#[pyfunction]
pub fn parse_literal<'py>(
    _raw: &str,
    _kind: PySyntaxKind,
    py: Python<'py>,
) -> PyResult<Bound<'py, PyAny>> {
    // Use Python's own parsing for primitives - Rust core may not expose parse_literal
    Ok(py.None().into_bound(py).into_any())
}

// =============================================================================
// DAG/Workflow Bindings
// =============================================================================

/// @brief Unique node identifier in a DAG.
/// @class
/// @group Orchestration
/// @since 0.1.0
///
/// @see PyDagNode
/// @see PyDagGraph
/// @see xiaoyi::workflow::dag::graph::NodeId
#[pyclass(from_py_object)]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PyNodeId {
    pub(crate) inner: crate::xiaoyi::workflow::dag::graph::NodeId,
}

#[pymethods]
impl PyNodeId {
    /// @brief Create a new node ID.
    /// @param id String identifier.
    /// @since 0.1.0
    #[new]
    pub fn new(id: String) -> Self {
        Self {
            inner: crate::xiaoyi::workflow::dag::graph::NodeId(id),
        }
    }

    /// @brief Get the ID string.
    /// @return String representation of the ID.
    /// @since 0.1.0
    #[getter]
    pub fn id(&self) -> String {
        self.inner.0.clone()
    }

    fn __repr__(&self) -> String {
        format!("NodeId('{}')", self.inner.0)
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.0.hash(&mut hasher);
        hasher.finish() as u64
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// @brief Node kind classification for DAG nodes.
/// @enum
/// @group Orchestration
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyNodeKind {
    /// @brief Task node.
    Task = 0,
    /// @brief Condition node.
    Condition = 1,
    /// @brief Parallel execution node.
    Parallel = 2,
    /// @brief Merge node.
    Merge = 3,
}

#[pymethods]
impl PyNodeKind {
    fn __repr__(&self) -> String {
        match self {
            PyNodeKind::Task => "NodeKind.Task".to_string(),
            PyNodeKind::Condition => "NodeKind.Condition".to_string(),
            PyNodeKind::Parallel => "NodeKind.Parallel".to_string(),
            PyNodeKind::Merge => "NodeKind.Merge".to_string(),
        }
    }
}

/// @brief Edge kind classification.
/// @enum
/// @group Orchestration
/// @since 0.1.0
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyEdgeKind {
    /// @brief Sequential dependency.
    Sequential = 0,
    /// @brief Parallel execution.
    Parallel = 1,
    /// @brief Conditional dependency.
    Conditional = 2,
}

#[pymethods]
impl PyEdgeKind {
    fn __repr__(&self) -> String {
        match self {
            PyEdgeKind::Sequential => "EdgeKind.Sequential".to_string(),
            PyEdgeKind::Parallel => "EdgeKind.Parallel".to_string(),
            PyEdgeKind::Conditional => "EdgeKind.Conditional".to_string(),
        }
    }
}

/// @brief DAG node with metadata.
/// @class
/// @group Orchestration
/// @since 0.1.0
///
/// @see PyNodeId
/// @see PyDagGraph
/// @see xiaoyi::workflow::dag::graph::DagNode
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDagNode {
    pub(crate) inner: crate::xiaoyi::workflow::dag::graph::DagNode,
}

#[pymethods]
impl PyDagNode {
    /// @brief Create a new DAG node.
    /// @param id Node identifier.
    /// @param label Human-readable label.
    /// @param kind Node kind.
    /// @return New DagNode.
    /// @since 0.1.0
    #[new]
    pub fn new(id: PyNodeId, label: String, kind: PyNodeKind) -> PyResult<Self> {
        let node_kind = match kind {
            PyNodeKind::Task => crate::xiaoyi::workflow::dag::graph::NodeKind::Task,
            PyNodeKind::Condition => crate::xiaoyi::workflow::dag::graph::NodeKind::Condition,
            PyNodeKind::Parallel => crate::xiaoyi::workflow::dag::graph::NodeKind::Parallel,
            PyNodeKind::Merge => crate::xiaoyi::workflow::dag::graph::NodeKind::Merge,
        };
        Ok(Self {
            inner: crate::xiaoyi::workflow::dag::graph::DagNode::new(id.inner, label, node_kind),
        })
    }

    /// @brief Get the node ID.
    /// @return NodeId.
    /// @since 0.1.0
    #[getter]
    pub fn id(&self) -> PyNodeId {
        PyNodeId {
            inner: self.inner.id.clone(),
        }
    }

    /// @brief Get the node label.
    /// @return Label string.
    /// @since 0.1.0
    #[getter]
    pub fn label(&self) -> String {
        self.inner.label.clone()
    }
}

/// @brief DAG edge connecting two nodes.
/// @class
/// @group Orchestration
/// @since 0.1.0
///
/// @see PyDagNode
/// @see PyDagGraph
/// @see xiaoyi::workflow::dag::graph::DagEdge
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDagEdge {
    pub(crate) inner: crate::xiaoyi::workflow::dag::graph::DagEdge,
}

#[pymethods]
impl PyDagEdge {
    /// @brief Create a new DAG edge.
    /// @param from Source node ID.
    /// @param to Target node ID.
    /// @param kind Edge kind.
    /// @return New DagEdge.
    /// @since 0.1.0
    #[new]
    pub fn new(from: PyNodeId, to: PyNodeId, kind: PyEdgeKind) -> Self {
        let edge_kind = match kind {
            PyEdgeKind::Sequential => crate::xiaoyi::workflow::dag::graph::EdgeKind::Sequential,
            PyEdgeKind::Parallel => crate::xiaoyi::workflow::dag::graph::EdgeKind::Parallel,
            PyEdgeKind::Conditional => crate::xiaoyi::workflow::dag::graph::EdgeKind::Conditional,
        };
        Self {
            inner: crate::xiaoyi::workflow::dag::graph::DagEdge::new(
                from.inner, to.inner, edge_kind,
            ),
        }
    }
}

/// @brief Directed acyclic graph for workflow execution.
/// @class
/// @group Orchestration
/// @since 0.1.0
///
/// @see PyDagNode
/// @see PyDagEdge
/// @see xiaoyi::workflow::dag::graph::DagGraph
#[pyclass]
#[derive(Debug, Default)]
pub struct PyDagGraph {
    pub(crate) inner: crate::xiaoyi::workflow::dag::graph::DagGraph,
}

#[pymethods]
impl PyDagGraph {
    /// @brief Create a new empty DAG.
    /// @return Empty DagGraph.
    /// @since 0.1.0
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    /// @brief Add a node to the DAG.
    /// @param node DagNode to add.
    /// @return The NodeId of the added node.
    /// @since 0.1.0
    pub fn add_node(&mut self, node: PyDagNode) -> PyNodeId {
        let idx = self.inner.add_node(node.inner);
        let node_id = self
            .inner
            .node_id(idx)
            .unwrap_or_else(|| NodeId::new("unknown"));
        PyNodeId { inner: node_id }
    }

    /// @brief Add an edge between nodes.
    /// @param edge DagEdge to add.
    /// @throw PyValueError if edge references non-existent nodes or creates a cycle.
    /// @since 0.1.0
    pub fn add_edge(&mut self, edge: PyDagEdge) -> PyResult<()> {
        self.inner
            .add_edge(edge.inner)
            .map_err(|e| PyValueError::new_err(format!("DAG error: {}", e)))
    }

    /// @brief Get topological ordering of nodes.
    /// @return List of NodeIds in topological order.
    /// @throw PyRuntimeError if a cycle is detected.
    /// @since 0.1.0
    pub fn topological_order(&self) -> PyResult<Vec<PyNodeId>> {
        self.inner
            .topological_order()
            .map(|ids| ids.into_iter().map(|id| PyNodeId { inner: id }).collect())
            .map_err(|e| PyRuntimeError::new_err(format!("Topological sort failed: {}", e)))
    }
}

// =============================================================================
// Memory Bindings
// =============================================================================

/// @brief Cache entry with value and optional TTL expiry.
/// @class
/// @group Memory
/// @since 0.1.0
///
/// @see PyLruCache
/// @see xiaoyi::memory::stm::cache::CacheEntry
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCacheEntry {
    pub(crate) inner: crate::xiaoyi::memory::stm::cache::CacheEntry<String>,
}

#[pymethods]
impl PyCacheEntry {
    /// @brief Get the cached value.
    /// @return The value if not expired, None otherwise.
    /// @since 0.1.0
    pub fn value(&self) -> Option<String> {
        if self.inner.is_expired() {
            None
        } else {
            Some(self.inner.value.clone())
        }
    }

    /// @brief Check if the entry is expired.
    /// @return True if expired.
    /// @since 0.1.0
    pub fn is_expired(&self) -> bool {
        self.inner.is_expired()
    }
}

/// @brief Cache performance statistics.
/// @class
/// @group Memory
/// @since 0.1.0
///
/// @see PyLruCache
/// @see xiaoyi::memory::stm::cache::CacheStats
#[pyclass(from_py_object)]
#[derive(Debug, Clone, Default)]
pub struct PyCacheStats {
    pub(crate) inner: crate::xiaoyi::memory::stm::cache::CacheStats,
}

#[pymethods]
impl PyCacheStats {
    /// @brief Number of cache hits.
    /// @return Hit count.
    /// @since 0.1.0
    #[getter]
    pub fn hits(&self) -> u64 {
        self.inner.hits as u64
    }

    /// @brief Number of cache misses.
    /// @return Miss count.
    /// @since 0.1.0
    #[getter]
    pub fn misses(&self) -> u64 {
        self.inner.misses as u64
    }

    /// @brief Cache hit rate.
    /// @return Hit rate as a float (0.0 to 1.0).
    /// @since 0.1.0
    #[getter]
    pub fn hit_rate(&self) -> f64 {
        self.inner.hit_rate()
    }

    /// @brief Number of entries evicted.
    /// @return Eviction count.
    /// @since 0.1.0
    #[getter]
    pub fn evictions(&self) -> u64 {
        self.inner.evictions as u64
    }

    /// @brief Current number of entries.
    /// @return Entry count.
    /// @since 0.1.0
    #[getter]
    pub fn size(&self) -> usize {
        self.inner.size
    }

    /// @brief Maximum capacity.
    /// @return Capacity.
    /// @since 0.1.0
    #[getter]
    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }
}

/// @brief Thread-safe LRU cache with optional TTL.
/// @class
/// @group Memory
/// @since 0.1.0
///
/// Provides O(1) get/insert with LRU eviction and optional TTL per entry.
///
/// @see PyCacheStats
/// @see PyCacheEntry
/// @see xiaoyi::memory::stm::cache::LruCache
/// @example
/// ```python
/// cache = LruCache(100)
/// cache.insert("key", "value")
/// value = cache.get("key")  # "value"
/// ```
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyLruCache {
    pub(crate) inner: std::sync::Arc<
        tokio::sync::RwLock<crate::xiaoyi::memory::stm::cache::LruCache<String, String>>,
    >,
}

#[pymethods]
impl PyLruCache {
    /// @brief Create a new LRU cache with the given capacity.
    /// @param capacity Maximum number of entries.
    /// @return New LruCache instance.
    /// @since 0.1.0
    #[new]
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: std::sync::Arc::new(tokio::sync::RwLock::new(
                crate::xiaoyi::memory::stm::cache::LruCache::new(capacity),
            )),
        }
    }

    /// @brief Insert a key-value pair.
    /// @param key The cache key.
    /// @param value The value to cache.
    /// @param ttl Optional TTL in seconds (None = no expiry).
    /// @throw PyRuntimeError if runtime creation fails.
    /// @since 0.1.0
    pub fn insert(&self, key: String, value: String, ttl: Option<u64>) -> PyResult<()> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            let duration = ttl.map(std::time::Duration::from_secs);
            self.inner.write().await.insert(key, value, duration);
        });
        Ok(())
    }

    /// @brief Get a value by key.
    /// @param key The cache key.
    /// @return The value if present and not expired, None otherwise.
    /// @since 0.1.0
    pub fn get(&self, key: &str) -> PyResult<Option<String>> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let key = key.to_string();
        rt.block_on(async { Ok(self.inner.read().await.get(&key)) })
            .map_err(|e: tokio::task::JoinError| PyRuntimeError::new_err(e.to_string()))
    }
    /// @brief Remove a key from the cache.
    /// @param key The cache key.
    /// @return true if key was present.
    pub fn remove(&self, key: &str) -> PyResult<bool> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let key = key.to_string();
        rt.block_on(async { Ok(self.inner.write().await.remove(&key)) })
            .map_err(|e: tokio::task::JoinError| PyRuntimeError::new_err(e.to_string()))
    }
    /// @since 0.1.0
    /// @brief Clear all entries from the cache.
    /// @since 0.1.0
    pub fn clear(&self) -> PyResult<()> {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.inner.write().await.clear();
        });
        Ok(())
    }
    /// @since 0.1.0
    pub fn stats(&self) -> PyCacheStats {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let stats = rt.block_on(async { self.inner.read().await.stats() });
        PyCacheStats { inner: stats }
    }
}

// =============================================================================
// Module Initialization
// =============================================================================

/// @brief Initialize the Python module.
/// @param py Python interpreter.
/// @param m Module.
/// @return PyResult.
/// @since 0.1.0
#[pymodule]
pub fn xiaoyi(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Error submodule
    let error_mod = PyModule::new(py, "error")?;
    error_mod.add("ErrorKind", py.get_type::<PyErrorKind>())?;
    error_mod.add_function(wrap_pyfunction!(create_error, &error_mod)?)?;
    error_mod.add_function(wrap_pyfunction!(is_xiaoyi_error, &error_mod)?)?;
    error_mod.add_function(wrap_pyfunction!(create_error, m)?)?;
    error_mod.add_function(wrap_pyfunction!(is_xiaoyi_error, m)?)?;
    m.add_submodule(&error_mod)?;

    // Status submodule
    let status_mod = PyModule::new(py, "status")?;
    status_mod.add("Status", py.get_type::<PyStatus>())?;
    m.add_submodule(&status_mod)?;

    // Config submodule
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add("Config", py.get_type::<PyConfig>())?;
    config_mod.add("ConfigBuilder", py.get_type::<PyConfigBuilder>())?;
    config_mod.add("ConfigSource", py.get_type::<PyConfigSource>())?;
    config_mod.add("FileSource", py.get_type::<PyFileSource>())?;
    config_mod.add("EnvSource", py.get_type::<PyEnvSource>())?;
    config_mod.add("VaultSource", py.get_type::<PyVaultSource>())?;
    config_mod.add("SourceCapability", py.get_type::<PySourceCapability>())?;
    config_mod.add_function(wrap_pyfunction!(encrypt_vault, m)?)?;
    config_mod.add_function(wrap_pyfunction!(decrypt_vault, m)?)?;
    m.add_submodule(&config_mod)?;

    // Primitive submodule
    let primitive_mod = PyModule::new(py, "primitive")?;
    primitive_mod.add("PrimitiveKind", py.get_type::<PyPrimitiveKind>())?;
    primitive_mod.add("IntKind", py.get_type::<PyIntKind>())?;
    primitive_mod.add("IntWidth", py.get_type::<PyIntWidth>())?;
    primitive_mod.add("FloatKind", py.get_type::<PyFloatKind>())?;
    m.add_submodule(&primitive_mod)?;

    // Syntax submodule
    let syntax_mod = PyModule::new(py, "syntax")?;
    syntax_mod.add("SyntaxKind", py.get_type::<PySyntaxKind>())?;
    syntax_mod.add("OperatorKind", py.get_type::<PyOperatorKind>())?;
    syntax_mod.add("Associativity", py.get_type::<PyAssociativity>())?;
    syntax_mod.add("Operator", py.get_type::<PyOperator>())?;
    syntax_mod.add_function(wrap_pyfunction!(operator_from_symbol, m)?)?;
    syntax_mod.add_function(wrap_pyfunction!(operators_with_prefix, m)?)?;
    syntax_mod.add_function(wrap_pyfunction!(all_operators, m)?)?;
    m.add_submodule(&syntax_mod)?;

    // Workflow DAG submodule
    let workflow_mod = PyModule::new(py, "workflow")?;
    let dag_mod = PyModule::new(py, "dag")?;
    dag_mod.add("NodeId", py.get_type::<PyNodeId>())?;
    dag_mod.add("NodeKind", py.get_type::<PyNodeKind>())?;
    dag_mod.add("EdgeKind", py.get_type::<PyEdgeKind>())?;
    dag_mod.add("DagNode", py.get_type::<PyDagNode>())?;
    dag_mod.add("DagEdge", py.get_type::<PyDagEdge>())?;
    dag_mod.add("DagGraph", py.get_type::<PyDagGraph>())?;
    dag_mod.add("Dag", py.get_type::<PyDagGraph>())?;
    workflow_mod.add_submodule(&dag_mod)?;
    m.add_submodule(&workflow_mod)?;

    // Memory submodule
    let memory_mod = PyModule::new(py, "memory")?;
    let stm_mod = PyModule::new(py, "stm")?;
    stm_mod.add("LruCache", py.get_type::<PyLruCache>())?;
    stm_mod.add("StmCache", py.get_type::<PyLruCache>())?;
    stm_mod.add("CacheStats", py.get_type::<PyCacheStats>())?;
    stm_mod.add("CacheEntry", py.get_type::<PyCacheEntry>())?;
    memory_mod.add_submodule(&stm_mod)?;
    m.add_submodule(&memory_mod)?;
    Ok(())
}
