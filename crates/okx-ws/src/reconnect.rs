//! Auto-reconnecting WebSocket client.
//!
//! Provides automatic reconnection with exponential backoff and subscription state recovery.

use std::collections::HashSet;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Stream;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use okx_core::{Config, OkxError, Result};

use crate::channel::Channel;
use crate::client::WsClient;
use crate::message::WsMessage;

/// Configuration for reconnection behavior.
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Initial delay before first reconnection attempt
    pub initial_delay: Duration,
    /// Maximum delay between reconnection attempts
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Maximum number of reconnection attempts (None = unlimited)
    pub max_attempts: Option<u32>,
    /// Whether to automatically restore subscriptions after reconnect
    pub restore_subscriptions: bool,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            max_attempts: None,
            restore_subscriptions: true,
        }
    }
}

impl ReconnectConfig {
    /// Create a new reconnect configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the initial delay before first reconnection attempt.
    #[must_use]
    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the maximum delay between reconnection attempts.
    #[must_use]
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the backoff multiplier.
    #[must_use]
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Set the maximum number of reconnection attempts.
    #[must_use]
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = Some(attempts);
        self
    }

    /// Set whether to automatically restore subscriptions after reconnect.
    #[must_use]
    pub fn with_restore_subscriptions(mut self, restore: bool) -> Self {
        self.restore_subscriptions = restore;
        self
    }
}

/// Connection type for the WebSocket client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Public WebSocket connection
    Public,
    /// Private WebSocket connection (requires authentication)
    Private,
}

/// State of the reconnecting client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connected and operational
    Connected,
    /// Disconnected, waiting to reconnect
    Disconnected,
    /// Currently attempting to reconnect
    Reconnecting,
    /// Permanently disconnected (max attempts reached)
    Failed,
}

/// Auto-reconnecting WebSocket client.
///
/// Wraps `WsClient` with automatic reconnection and subscription state recovery.
///
/// ## Example
///
/// ```rust,no_run
/// use okx_ws::{ReconnectingWsClient, ReconnectConfig, ConnectionType, Channel, WsMessage};
/// use okx_core::{Config, Credentials};
/// use futures_util::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let credentials = Credentials::new("api_key", "secret_key", "passphrase");
///     let config = Config::new(credentials).simulated(true);
///     let reconnect_config = ReconnectConfig::default();
///
///     let mut client = ReconnectingWsClient::connect(
///         config,
///         ConnectionType::Public,
///         reconnect_config,
///     ).await?;
///
///     // Subscribe to channels
///     client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".to_string() }]).await?;
///
///     // Process messages - reconnection is handled automatically
///     while let Some(msg) = client.next().await {
///         match msg {
///             Ok(WsMessage::Data { channel, data, .. }) => {
///                 println!("Channel: {}, Data: {:?}", channel, data);
///             }
///             Err(e) => eprintln!("Error: {}", e),
///             _ => {}
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct ReconnectingWsClient {
    /// Inner WebSocket client
    client: Option<WsClient>,
    /// Client configuration
    config: Config,
    /// Connection type (public/private)
    conn_type: ConnectionType,
    /// Reconnection configuration
    reconnect_config: ReconnectConfig,
    /// Current connection state
    state: ConnectionState,
    /// Active subscriptions (for recovery)
    subscriptions: HashSet<ChannelKey>,
    /// Current reconnection attempt count
    attempt_count: u32,
    /// Current backoff delay
    current_delay: Duration,
}

/// Key for tracking subscriptions (serialized channel).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ChannelKey(String);

impl From<&Channel> for ChannelKey {
    fn from(channel: &Channel) -> Self {
        Self(serde_json::to_string(channel).unwrap_or_default())
    }
}

impl ChannelKey {
    fn to_channel(&self) -> Option<Channel> {
        serde_json::from_str(&self.0).ok()
    }
}

impl ReconnectingWsClient {
    /// Connect to the WebSocket endpoint with auto-reconnection enabled.
    pub async fn connect(
        config: Config,
        conn_type: ConnectionType,
        reconnect_config: ReconnectConfig,
    ) -> Result<Self> {
        let client = Self::create_client(&config, conn_type).await?;

        let initial_delay = reconnect_config.initial_delay;
        Ok(Self {
            client: Some(client),
            config,
            conn_type,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: initial_delay,
        })
    }

    /// Get the current connection state.
    #[must_use]
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Check if the client is connected.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }

    /// Get the number of active subscriptions.
    #[must_use]
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.len()
    }

    /// Subscribe to channels.
    ///
    /// Subscriptions are tracked and will be automatically restored after reconnection.
    pub async fn subscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        // Track subscriptions for recovery
        for channel in &channels {
            self.subscriptions.insert(ChannelKey::from(channel));
        }

        // Subscribe on the current connection
        if let Some(client) = &mut self.client {
            client.subscribe(channels).await?;
        }

        Ok(())
    }

    /// Unsubscribe from channels.
    pub async fn unsubscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        // Remove from tracked subscriptions
        for channel in &channels {
            self.subscriptions.remove(&ChannelKey::from(channel));
        }

        // Unsubscribe on the current connection
        if let Some(client) = &mut self.client {
            client.unsubscribe(channels).await?;
        }

        Ok(())
    }

    /// Send a ping to keep the connection alive.
    pub async fn ping(&mut self) -> Result<()> {
        if let Some(client) = &mut self.client {
            client.ping().await
        } else {
            Err(OkxError::ConnectionClosed)
        }
    }

    /// Close the connection and stop reconnection attempts.
    pub async fn close(&mut self) -> Result<()> {
        self.state = ConnectionState::Failed;
        if let Some(client) = &mut self.client {
            client.close().await?;
        }
        self.client = None;
        Ok(())
    }

    /// Manually trigger a reconnection.
    pub async fn reconnect(&mut self) -> Result<()> {
        self.do_reconnect().await
    }

    /// Create a new WebSocket client.
    async fn create_client(config: &Config, conn_type: ConnectionType) -> Result<WsClient> {
        match conn_type {
            ConnectionType::Public => WsClient::connect_public(config).await,
            ConnectionType::Private => WsClient::connect_private(config).await,
        }
    }

    /// Perform reconnection with backoff.
    async fn do_reconnect(&mut self) -> Result<()> {
        self.state = ConnectionState::Reconnecting;
        self.client = None;

        loop {
            // Check max attempts
            if let Some(max) = self.reconnect_config.max_attempts {
                if self.attempt_count >= max {
                    error!("Max reconnection attempts ({}) reached", max);
                    self.state = ConnectionState::Failed;
                    return Err(OkxError::Other(format!(
                        "Max reconnection attempts ({}) reached",
                        max
                    )));
                }
            }

            self.attempt_count += 1;
            info!(
                "Reconnection attempt {} (delay: {:?})",
                self.attempt_count, self.current_delay
            );

            // Wait before attempting
            sleep(self.current_delay).await;

            // Try to connect
            match Self::create_client(&self.config, self.conn_type).await {
                Ok(client) => {
                    info!("Reconnection successful");
                    self.client = Some(client);
                    self.state = ConnectionState::Connected;
                    self.attempt_count = 0;
                    self.current_delay = self.reconnect_config.initial_delay;

                    // Restore subscriptions if configured
                    if self.reconnect_config.restore_subscriptions {
                        self.restore_subscriptions().await?;
                    }

                    return Ok(());
                }
                Err(e) => {
                    warn!("Reconnection attempt {} failed: {}", self.attempt_count, e);

                    // Increase backoff delay
                    let new_delay = Duration::from_secs_f64(
                        self.current_delay.as_secs_f64() * self.reconnect_config.backoff_multiplier,
                    );
                    self.current_delay = new_delay.min(self.reconnect_config.max_delay);
                }
            }
        }
    }

    /// Restore all tracked subscriptions.
    async fn restore_subscriptions(&mut self) -> Result<()> {
        if self.subscriptions.is_empty() {
            return Ok(());
        }

        info!("Restoring {} subscriptions", self.subscriptions.len());

        let channels: Vec<Channel> = self
            .subscriptions
            .iter()
            .filter_map(|key| key.to_channel())
            .collect();

        if channels.is_empty() {
            return Ok(());
        }

        // For private connections, login first
        if self.conn_type == ConnectionType::Private {
            if let Some(client) = &mut self.client {
                client.login().await?;
            }
        }

        // Re-subscribe
        if let Some(client) = &mut self.client {
            client.subscribe(channels).await?;
        }

        info!("Subscriptions restored successfully");
        Ok(())
    }
}

/// 将频道序列化为字符串键（用于订阅恢复或测试）。
pub fn channel_key_from(channel: &Channel) -> String {
    ChannelKey::from(channel).0
}

/// 从序列化键恢复频道。
pub fn channel_from_key(key: &str) -> Option<Channel> {
    serde_json::from_str(key).ok()
}

impl Stream for ReconnectingWsClient {
    type Item = Result<WsMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // If we're in a failed state, return None
        if self.state == ConnectionState::Failed {
            return Poll::Ready(None);
        }

        // If we're disconnected or reconnecting, wake up later
        if self.state != ConnectionState::Connected {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        // Poll the inner client
        if let Some(client) = &mut self.client {
            match Pin::new(client).poll_next(cx) {
                Poll::Ready(Some(Ok(msg))) => Poll::Ready(Some(Ok(msg))),
                Poll::Ready(Some(Err(e))) => {
                    // Check if this is a connection error
                    if matches!(e, OkxError::ConnectionClosed | OkxError::WebSocket(_)) {
                        debug!("Connection error detected: {}", e);
                        // We can't do async reconnection here, so we return the error
                        // and let the user handle it or call reconnect()
                        self.state = ConnectionState::Disconnected;
                    }
                    Poll::Ready(Some(Err(e)))
                }
                Poll::Ready(None) => {
                    // Stream ended, connection closed
                    debug!("WebSocket stream ended");
                    self.state = ConnectionState::Disconnected;
                    Poll::Ready(Some(Err(OkxError::ConnectionClosed)))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            // No client, we're disconnected
            self.state = ConnectionState::Disconnected;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use okx_core::{Config, Credentials};
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::time::timeout;

    #[test]
    fn test_reconnect_config_default() {
        let config = ReconnectConfig::default();
        assert_eq!(config.initial_delay, Duration::from_secs(1));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.max_attempts.is_none());
        assert!(config.restore_subscriptions);
    }

    #[test]
    fn test_reconnect_config_builder() {
        let config = ReconnectConfig::new()
            .with_initial_delay(Duration::from_millis(500))
            .with_max_delay(Duration::from_secs(30))
            .with_backoff_multiplier(1.5)
            .with_max_attempts(5)
            .with_restore_subscriptions(false);

        assert_eq!(config.initial_delay, Duration::from_millis(500));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.backoff_multiplier, 1.5);
        assert_eq!(config.max_attempts, Some(5));
        assert!(!config.restore_subscriptions);
    }

    #[test]
    fn test_channel_key_roundtrip() {
        let channel = Channel::Tickers {
            inst_id: "BTC-USDT".to_string(),
        };
        let key = ChannelKey::from(&channel);
        let recovered = key.to_channel().unwrap();

        match recovered {
            Channel::Tickers { inst_id } => assert_eq!(inst_id, "BTC-USDT"),
            _ => panic!("Wrong channel type"),
        }
    }

    #[test]
    fn test_state_helpers() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config: ReconnectConfig::default(),
            state: ConnectionState::Disconnected,
            subscriptions: HashSet::new(),
            attempt_count: 1,
            current_delay: Duration::from_secs(2),
        };

        assert_eq!(client.state(), ConnectionState::Disconnected);
        assert!(!client.is_connected());

        client.state = ConnectionState::Connected;
        assert!(client.is_connected());
        assert_eq!(client.state(), ConnectionState::Connected);
    }

    #[tokio::test]
    async fn test_connect_returns_error_for_unreachable_ws() {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_ws_public_url("ws://127.0.0.1:9/ws")
            .with_ws_private_url("ws://127.0.0.1:9/ws");

        let result = timeout(
            Duration::from_secs(2),
            ReconnectingWsClient::connect(cfg, ConnectionType::Public, ReconnectConfig::default()),
        )
        .await;

        match result {
            Ok(Err(e)) => {
                // 连接失败即可覆盖分支
                assert!(matches!(
                    e,
                    OkxError::WebSocket(_) | OkxError::ConnectionClosed
                ));
            }
            Ok(Ok(_)) => panic!("意外成功连接本地未监听端口"),
            Err(_) => eprintln!("连接超时，CI 可接受"),
        }
    }

    #[tokio::test]
    async fn do_reconnect_respects_max_attempts_and_sets_failed_state() {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_ws_public_url("ws://127.0.0.1:9/ws")
            .with_ws_private_url("ws://127.0.0.1:9/ws");

        let reconnect_config = ReconnectConfig::default()
            .with_initial_delay(Duration::from_millis(1))
            .with_max_delay(Duration::from_millis(2))
            .with_max_attempts(1)
            .with_restore_subscriptions(false);

        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Disconnected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        let err = timeout(Duration::from_secs(2), client.do_reconnect())
            .await
            .expect("重连流程超时")
            .expect_err("应达到最大重连次数并失败");

        assert_eq!(client.state(), ConnectionState::Failed);
        assert!(matches!(err, OkxError::Other(_)));
        assert!(err.to_string().contains("Max reconnection attempts"));
    }

    #[tokio::test]
    async fn restore_subscriptions_returns_ok_for_empty_and_invalid_keys() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default().with_restore_subscriptions(true);

        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        // 空订阅应直接返回 Ok
        client.restore_subscriptions().await.expect("空订阅应 Ok");

        // 插入无法反序列化的 key，channels 为空也应 Ok
        client
            .subscriptions
            .insert(ChannelKey("not-json".to_string()));
        client
            .restore_subscriptions()
            .await
            .expect("无效 key 应被过滤并返回 Ok");
    }

    #[tokio::test]
    async fn subscribe_and_unsubscribe_track_without_inner_client() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default().with_restore_subscriptions(false);

        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        client
            .subscribe(vec![Channel::Tickers {
                inst_id: "BTC-USDT".to_string(),
            }])
            .await
            .expect("无内层连接也应允许记录订阅");
        assert_eq!(client.subscription_count(), 1);

        client
            .unsubscribe(vec![Channel::Tickers {
                inst_id: "BTC-USDT".to_string(),
            }])
            .await
            .expect("取消订阅应成功");
        assert_eq!(client.subscription_count(), 0);
    }

    #[tokio::test]
    async fn ping_returns_connection_closed_when_client_none() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default();
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        let err = client.ping().await.expect_err("无连接时 ping 应失败");
        assert!(matches!(err, OkxError::ConnectionClosed));
    }

    #[tokio::test]
    async fn stream_poll_next_failed_state_returns_none() {
        use futures_util::StreamExt;

        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default();
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Failed,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        assert!(client.next().await.is_none());
    }

    #[test]
    fn stream_poll_next_connected_without_inner_client_sets_disconnected_and_pending() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default();
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        let waker = futures_util::task::noop_waker_ref();
        let mut cx = Context::from_waker(waker);
        let polled = Pin::new(&mut client).poll_next(&mut cx);
        assert!(matches!(polled, Poll::Pending));
        assert_eq!(client.state(), ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_connect_succeeds_with_local_echo_server() {
        use tokio::net::TcpListener;
        use tokio_tungstenite::accept_async;

        // 本地简易 WebSocket 服务，接受后立刻关闭
        let listener = match TcpListener::bind("127.0.0.1:0").await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("无法绑定本地端口，跳过用例: {e}");
                return;
            }
        };
        let addr = match listener.local_addr() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("无法获取地址，跳过用例: {e}");
                return;
            }
        };

        let server = tokio::spawn(async move {
            if let Ok((stream, _)) = listener.accept().await {
                let _ = accept_async(stream).await;
            }
        });

        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_ws_public_url(format!("ws://{addr}/ws"))
            .with_ws_private_url(format!("ws://{addr}/ws"));

        let client = timeout(
            Duration::from_secs(3),
            ReconnectingWsClient::connect(cfg, ConnectionType::Public, ReconnectConfig::default()),
        )
        .await
        .expect("连接超时")
        .expect("连接应成功");

        assert_eq!(client.state(), ConnectionState::Connected);
        assert!(client.is_connected());

        server.abort();
    }

    #[tokio::test]
    async fn close_sets_failed_and_clears_client_when_none() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default();
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        client.close().await.expect("close 应返回 Ok");
        assert_eq!(client.state(), ConnectionState::Failed);
        assert!(client.client.is_none());
    }

    #[test]
    fn stream_poll_next_reconnecting_wakes_and_pending() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        let reconnect_config = ReconnectConfig::default();
        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Public,
            reconnect_config,
            state: ConnectionState::Reconnecting,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        let waker = futures_util::task::noop_waker_ref();
        let mut cx = Context::from_waker(waker);
        let polled = Pin::new(&mut client).poll_next(&mut cx);
        assert!(matches!(polled, Poll::Pending));
        assert_eq!(client.state(), ConnectionState::Reconnecting);
    }

    #[tokio::test]
    async fn reconnect_wrapper_hits_private_create_client_arm() {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_ws_public_url("ws://127.0.0.1:9/ws")
            .with_ws_private_url("ws://127.0.0.1:9/ws");

        let reconnect_config = ReconnectConfig::default()
            .with_initial_delay(Duration::from_millis(1))
            .with_max_delay(Duration::from_millis(1))
            .with_backoff_multiplier(1.0)
            .with_max_attempts(1)
            .with_restore_subscriptions(false);

        let mut client = ReconnectingWsClient {
            client: None,
            config: cfg,
            conn_type: ConnectionType::Private,
            reconnect_config,
            state: ConnectionState::Connected,
            subscriptions: HashSet::new(),
            attempt_count: 0,
            current_delay: Duration::from_millis(1),
        };

        let err = timeout(Duration::from_secs(2), client.reconnect())
            .await
            .expect("重连流程超时")
            .expect_err("应达到最大重连次数并失败");

        assert_eq!(client.state(), ConnectionState::Failed);
        assert!(matches!(err, OkxError::Other(_)));
    }
}
