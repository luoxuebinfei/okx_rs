//! Python bindings for ResponseMeta.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use okx_rest::ResponseMeta;

/// HTTP response metadata.
///
/// Contains status code and headers from the HTTP response.
/// Useful for implementing rate limit handling.
///
/// Example:
///     >>> data, meta = await client.get_public_with_meta("/api/v5/public/time")
///     >>> print(meta.status)
///     >>> print(meta.rate_limit_remaining)
#[pyclass(name = "ResponseMeta")]
#[derive(Clone)]
pub struct PyResponseMeta {
    pub(crate) inner: ResponseMeta,
}

impl From<ResponseMeta> for PyResponseMeta {
    fn from(inner: ResponseMeta) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyResponseMeta {
    /// HTTP status code.
    #[getter]
    fn status(&self) -> u16 {
        self.inner.status()
    }

    /// Check if the response was successful (2xx status).
    #[getter]
    fn is_success(&self) -> bool {
        self.inner.is_success()
    }

    /// Get a header value by name (case-insensitive).
    ///
    /// Args:
    ///     name: Header name
    ///
    /// Returns:
    ///     Header value or None if not found
    fn header(&self, name: &str) -> Option<String> {
        self.inner.header(name).map(String::from)
    }

    /// Get all headers as a dictionary.
    fn headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (k, v) in self.inner.headers() {
            dict.set_item(k, v)?;
        }
        Ok(dict)
    }

    /// Rate limit remaining (if present in headers).
    #[getter]
    fn rate_limit_remaining(&self) -> Option<u32> {
        self.inner.rate_limit_remaining()
    }

    /// Rate limit total (if present in headers).
    #[getter]
    fn rate_limit_limit(&self) -> Option<u32> {
        self.inner.rate_limit_limit()
    }

    /// Rate limit reset time in seconds (if present in headers).
    #[getter]
    fn rate_limit_reset(&self) -> Option<u64> {
        self.inner.rate_limit_reset()
    }

    fn __repr__(&self) -> String {
        format!(
            "ResponseMeta(status={}, is_success={})",
            self.inner.status(),
            self.inner.is_success()
        )
    }
}
