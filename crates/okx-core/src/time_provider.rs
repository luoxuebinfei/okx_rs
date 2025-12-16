//! Timestamp provider trait for time synchronization.
//!
//! This module defines the `TimestampProvider` trait that allows
//! injecting custom time sources for API authentication.

use chrono::{DateTime, Utc};

/// Trait for providing timestamps for API authentication.
///
/// Implementations can provide server-synchronized time or local time.
/// The timestamp is used for signing REST requests and WebSocket login.
pub trait TimestampProvider: Send + Sync {
    /// Get the current timestamp as ISO 8601 string.
    ///
    /// Format: `2024-01-01T12:00:00.000Z`
    fn timestamp_iso(&self) -> String;

    /// Get the current timestamp as Unix seconds (for WebSocket login).
    fn timestamp_unix_secs(&self) -> i64;
}

/// Default timestamp provider using local system time.
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalTimeProvider;

impl TimestampProvider for LocalTimeProvider {
    fn timestamp_iso(&self) -> String {
        Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    }

    fn timestamp_unix_secs(&self) -> i64 {
        Utc::now().timestamp()
    }
}

/// Timestamp provider with a fixed offset from local time.
///
/// This is useful when you know the clock drift between client and server.
#[derive(Debug, Clone, Copy)]
pub struct OffsetTimeProvider {
    /// Offset in milliseconds (server time minus local time)
    offset_ms: i64,
}

impl OffsetTimeProvider {
    /// Create a new offset time provider.
    ///
    /// ## Arguments
    ///
    /// * `offset_ms` - Offset in milliseconds (server time minus local time).
    ///   Positive means server is ahead, negative means server is behind.
    #[must_use]
    pub const fn new(offset_ms: i64) -> Self {
        Self { offset_ms }
    }

    /// Get the current offset in milliseconds.
    #[must_use]
    pub const fn offset_ms(&self) -> i64 {
        self.offset_ms
    }

    /// Get the adjusted current time.
    #[allow(clippy::trivially_copy_pass_by_ref)] // 保持与 trait 方法签名一致
    fn adjusted_now(&self) -> DateTime<Utc> {
        let now = Utc::now();
        now + chrono::Duration::milliseconds(self.offset_ms)
    }
}

impl TimestampProvider for OffsetTimeProvider {
    fn timestamp_iso(&self) -> String {
        self.adjusted_now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string()
    }

    fn timestamp_unix_secs(&self) -> i64 {
        self.adjusted_now().timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_time_provider_returns_valid_iso_format() {
        let provider = LocalTimeProvider;
        let ts = provider.timestamp_iso();
        // Should match format like "2024-01-01T12:00:00.000Z"
        assert!(ts.ends_with('Z'));
        assert!(ts.contains('T'));
        assert_eq!(ts.len(), 24);
    }

    #[test]
    fn offset_time_provider_applies_offset() {
        let offset_ms = 5000; // 5 seconds ahead
        let provider = OffsetTimeProvider::new(offset_ms);

        let local = LocalTimeProvider;
        let local_secs = local.timestamp_unix_secs();
        let offset_secs = provider.timestamp_unix_secs();

        // Should be approximately 5 seconds ahead (allow 1 second tolerance)
        let diff = offset_secs - local_secs;
        assert!((4..=6).contains(&diff), "diff was {diff}");
    }

    #[test]
    fn negative_offset_works() {
        let offset_ms = -3000; // 3 seconds behind
        let provider = OffsetTimeProvider::new(offset_ms);

        let local = LocalTimeProvider;
        let local_secs = local.timestamp_unix_secs();
        let offset_secs = provider.timestamp_unix_secs();

        let diff = local_secs - offset_secs;
        assert!((2..=4).contains(&diff), "diff was {diff}");
    }
}
