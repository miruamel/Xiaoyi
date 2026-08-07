#![cfg(feature = "python")]
//! Python bindings (PyO3).

use pyo3::prelude::*;

/// Xiaoyi Python module entry.
#[pymodule]
pub fn xiaoyi(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("xiaoyi_error", m.py().get_type::<XiaoyiError>())?;
    Ok(())
}

#[pyclass]
#[derive(Clone)]
pub struct XiaoyiError {
    #[pyo3(get)]
    pub(crate) msg: String,
}

#[pymethods]
impl XiaoyiError {
    #[new]
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}