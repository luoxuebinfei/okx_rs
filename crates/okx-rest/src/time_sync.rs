//! Time synchronization with OKX server.
//!
//! This module provides `TimeSync` for maintaining clock synchronization
//! with the OKX server, which is essential for API authentication.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use okx_core::{OkxError, Result, TimestampProvider};
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::api::public::endpoints;
use crate::OkxRestClient;

/// Time synchronization component for OKX API.
///
/// `TimeSync` maintains the clock offset between local time and OKX server time.
/// It can be used as a `TimestampProvider` for REST and WebSocket authentication.
///
/// ## Example
///
/// ```rust,no_run
/// use okx_rest::{OkxRestClient, TimeSync};
/// use okx_core::{Config, Credentials, TimestampProvider};
///
/// # async fn example() -> okx_core::Result<()> {
/// let config = Config::new(Credentials::new("key", "secret", "pass"));
/// let client = OkxRestClient::new(config);
///
/// // Create and sync
/// let time_sync = TimeSync::new(client);
/// time_sync.sync().await?;
///
/// // Use as timestamp provider
/// let timestamp = time_sync.timestamp_iso();
/// println!("Server-synced time: {}", timestamp);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct TimeSync {
    client: OkxRestClient,
    /// Offset in milliseconds (server_time - local_time)
    offset_ms: Arc<AtomicI64>,
    /// Last sync time
    last_sync: Arc<RwLock<Option<DateTime<Utc>>>>,
    /// Refresh interval
    refresh_interval: Duration,
}

impl std::fmt::Debug for TimeSync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeSync")
            .field("offset_ms", &self.offset_ms.load(Ordering::Relaxed))
            .field("refresh_interval", &self.refresh_interval)
            .finish()
    }
}

impl TimeSync {
    /// Create a new `TimeSync` with default refresh interval (30 seconds).
    #[must_use]
    pub fn new(client: OkxRestClient) -> Self {
        Self::with_refresh_interval(client, Duration::from_secs(30))
    }

    /// Create a new `TimeSync` with custom refresh interval.
    #[must_use]
    pub fn with_refresh_interval(client: OkxRestClient, refresh_interval: Duration) -> Self {
        Self {
            client,
            offset_ms: Arc::new(AtomicI64::new(0)),
            last_sync: Arc::new(RwLock::new(None)),
            refresh_interval,
        }
    }

    /// Synchronize with OKX server time.
    ///
    /// This method fetches the server time and calculates the clock offset.
    /// Call this before using the `TimeSync` as a timestamp provider.
    pub async fn sync(&self) -> Result<()> {
        let before = Utc::now();

        let response: serde_json::Value = self
            .client
            .get_public_raw(endpoints::TIME, None::<&()>)
            .await?;

        let after = Utc::now();

        // Parse the response: {"code":"0","msg":"","data":[{"ts":"1234567890123"}]}
        let ts_str = response
            .get("data")
            .and_then(|d| d.get(0))
            .and_then(|item| item.get("ts"))
            .and_then(|ts| ts.as_str())
            .ok_or_else(|| OkxError::Other("Invalid time response format".to_string()))?;

        let server_ts_ms: i64 = ts_str
            .parse()
            .map_err(|e| OkxError::Other(format!("Invalid timestamp: {e}")))?;

        // Estimate local time at server response (midpoint of request)
        let rtt = after - before;
        let local_estimate_ms = before.timestamp_millis() + rtt.num_milliseconds() / 2;

        let offset = server_ts_ms - local_estimate_ms;
        self.offset_ms.store(offset, Ordering::Relaxed);

        *self.last_sync.write().await = Some(Utc::now());

        debug!(
            offset_ms = offset,
            rtt_ms = rtt.num_milliseconds(),
            "Time synchronized with OKX server"
        );

        Ok(())
    }

    /// Synchronize if the last sync is older than the refresh interval.
    ///
    /// Returns `true` if a sync was performed.
    pub async fn sync_if_needed(&self) -> Result<bool> {
        let needs_sync = {
            let last = self.last_sync.read().await;
            match *last {
                None => true,
                Some(last_time) => {
                    let elapsed = Utc::now() - last_time;
                    elapsed.to_std().unwrap_or(Duration::ZERO) > self.refresh_interval
                }
            }
        };

        if needs_sync {
            self.sync().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get the current clock offset in milliseconds.
    ///
    /// Positive means server is ahead of local time.
    #[must_use]
    pub fn offset_ms(&self) -> i64 {
        self.offset_ms.load(Ordering::Relaxed)
    }

    /// Check if the time has been synchronized at least once.
    pub async fn is_synced(&self) -> bool {
        self.last_sync.read().await.is_some()
    }

    /// Get the last sync time.
    pub async fn last_sync_time(&self) -> Option<DateTime<Utc>> {
        *self.last_sync.read().await
    }

    /// Get the refresh interval.
    #[must_use]
    pub const fn refresh_interval(&self) -> Duration {
        self.refresh_interval
    }

    /// Get the adjusted current time.
    fn adjusted_now(&self) -> DateTime<Utc> {
        let offset = self.offset_ms.load(Ordering::Relaxed);
        Utc::now() + chrono::Duration::milliseconds(offset)
    }

    /// Start a background task that periodically syncs time.
    ///
    /// Returns a handle that can be used to stop the task.
    /// The task will run until the handle is dropped or `stop()` is called.
    #[must_use]
    pub fn start_background_sync(self: Arc<Self>) -> TimeSyncHandle {
        let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
        let time_sync = self.clone();

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut rx => {
                        debug!("TimeSync background task stopped");
                        break;
                    }
                    _ = tokio::time::sleep(time_sync.refresh_interval) => {
                        if let Err(e) = time_sync.sync().await {
                            warn!(error = %e, "Failed to sync time in background");
                        }
                    }
                }
            }
        });

        TimeSyncHandle {
            _stop_tx: tx,
            _task: handle,
        }
    }
}

impl TimestampProvider for TimeSync {
    fn timestamp_iso(&self) -> String {
        self.adjusted_now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string()
    }

    fn timestamp_unix_secs(&self) -> i64 {
        self.adjusted_now().timestamp()
    }
}

/// Handle for the background sync task.
///
/// The task will be stopped when this handle is dropped.
pub struct TimeSyncHandle {
    _stop_tx: tokio::sync::oneshot::Sender<()>,
    _task: tokio::task::JoinHandle<()>,
}

impl TimeSyncHandle {
    /// Stop the background sync task.
    pub fn stop(self) {
        // Dropping self will send the stop signal
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use okx_core::{Config, Credentials};

    fn test_client() -> OkxRestClient {
        let config = Config::new(Credentials::new("test", "test", "test"));
        OkxRestClient::new(config)
    }

    #[test]
    fn time_sync_default_refresh_interval() {
        let client = test_client();
        let ts = TimeSync::new(client);
        assert_eq!(ts.refresh_interval(), Duration::from_secs(30));
    }

    #[test]
    fn time_sync_custom_refresh_interval() {
        let client = test_client();
        let ts = TimeSync::with_refresh_interval(client, Duration::from_secs(60));
        assert_eq!(ts.refresh_interval(), Duration::from_secs(60));
    }

    #[test]
    fn time_sync_initial_offset_is_zero() {
        let client = test_client();
        let ts = TimeSync::new(client);
        assert_eq!(ts.offset_ms(), 0);
    }

    #[test]
    fn time_sync_provides_valid_timestamp_format() {
        let client = test_client();
        let ts = TimeSync::new(client);
        let iso = ts.timestamp_iso();
        assert!(iso.ends_with('Z'));
        assert!(iso.contains('T'));
        assert_eq!(iso.len(), 24);
    }
}
