//! Python bindings for TimeSync.

use pyo3::prelude::*;
use std::sync::Arc;

use okx_core::TimestampProvider;
use okx_rest::{OkxRestClient, TimeSync};

use crate::to_py_err;
use crate::types::PyConfig;

/// Time synchronization component for OKX API.
///
/// Maintains clock offset between local time and OKX server time.
/// Essential for API authentication when there's clock drift.
///
/// Example:
///     >>> from okx_py import TimeSync, Config, Credentials
///     >>> creds = Credentials("key", "secret", "pass")
///     >>> config = Config(creds, simulated=True)
///     >>> ts = TimeSync(config)
///     >>> await ts.sync()
///     >>> print(ts.timestamp_iso())
///     >>> print(ts.offset_ms())
#[pyclass(name = "TimeSync")]
pub struct PyTimeSync {
    pub(crate) inner: Arc<TimeSync>,
}

#[pymethods]
impl PyTimeSync {
    /// Create a new TimeSync instance.
    ///
    /// Args:
    ///     config: OKX configuration with credentials
    ///     refresh_interval_secs: Optional refresh interval in seconds (default: 30)
    #[new]
    #[pyo3(signature = (config, refresh_interval_secs=None))]
    fn new(config: &PyConfig, refresh_interval_secs: Option<u64>) -> Self {
        let client = OkxRestClient::new(config.inner.clone());
        let inner = match refresh_interval_secs {
            Some(secs) => {
                TimeSync::with_refresh_interval(client, std::time::Duration::from_secs(secs))
            }
            None => TimeSync::new(client),
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Synchronize with OKX server time.
    ///
    /// Fetches server time and calculates clock offset.
    /// Call this before using timestamp methods.
    ///
    /// Raises:
    ///     OkxError: If the sync request fails
    fn sync<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            inner.sync().await.map_err(to_py_err)?;
            Ok(())
        })
    }

    /// Synchronize if the last sync is older than refresh interval.
    ///
    /// Returns:
    ///     bool: True if a sync was performed
    ///
    /// Raises:
    ///     OkxError: If the sync request fails
    fn sync_if_needed<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            inner.sync_if_needed().await.map_err(to_py_err)
        })
    }

    /// Get the current clock offset in milliseconds.
    ///
    /// Positive means server is ahead of local time.
    ///
    /// Returns:
    ///     int: Offset in milliseconds
    fn offset_ms(&self) -> i64 {
        self.inner.offset_ms()
    }

    /// Get server-synchronized timestamp in ISO 8601 format.
    ///
    /// Format: "2024-01-01T12:00:00.000Z"
    ///
    /// Returns:
    ///     str: ISO 8601 timestamp
    fn timestamp_iso(&self) -> String {
        self.inner.timestamp_iso()
    }

    /// Get server-synchronized timestamp as Unix seconds.
    ///
    /// Returns:
    ///     int: Unix timestamp in seconds
    fn timestamp_unix_secs(&self) -> i64 {
        self.inner.timestamp_unix_secs()
    }

    /// Check if time has been synchronized at least once.
    ///
    /// Returns:
    ///     bool: True if synced
    fn is_synced<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { Ok(inner.is_synced().await) })
    }

    /// Get the refresh interval in seconds.
    ///
    /// Returns:
    ///     float: Refresh interval in seconds
    fn refresh_interval_secs(&self) -> f64 {
        self.inner.refresh_interval().as_secs_f64()
    }

    fn __repr__(&self) -> String {
        format!(
            "TimeSync(offset_ms={}, refresh_interval_secs={})",
            self.inner.offset_ms(),
            self.inner.refresh_interval().as_secs()
        )
    }
}
