//! WebSocket client implementation.
//!
//! Source: OKX API v5 WebSocket API
//! - <https://www.okx.com/docs-v5/en/#websocket-api>

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, Stream, StreamExt,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::time::{interval, Interval};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error as WsError, Message},
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

use okx_core::{Config, OkxError, Result, Signer};

use crate::channel::Channel;
use crate::message::{WsMessage, WsRequest};

/// WebSocket client for OKX exchange.
///
/// Provides streaming access to real-time market data and account updates.
struct WsClientInner<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    /// WebSocket write half
    sink: SplitSink<WebSocketStream<S>, Message>,
    /// WebSocket read half
    stream: SplitStream<WebSocketStream<S>>,
    /// Client configuration
    config: Config,
    /// Whether this is a private connection
    is_private: bool,
    /// Whether logged in (for private connections)
    is_logged_in: bool,
    /// Heartbeat interval
    heartbeat: Interval,
}

impl<S> WsClientInner<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn new(ws_stream: WebSocketStream<S>, config: Config, is_private: bool) -> Self {
        let (sink, stream) = ws_stream.split();
        Self {
            sink,
            stream,
            config,
            is_private,
            is_logged_in: false,
            heartbeat: interval(Duration::from_secs(25)),
        }
    }

    /// 处理 WebSocket 消息并转换为 WsMessage，供流或测试使用。
    pub fn handle_message(
        result: std::result::Result<Message, WsError>,
    ) -> Option<Result<WsMessage>> {
        match result {
            Ok(Message::Text(text)) => {
                debug!("Received: {}", text);
                Some(Ok(WsMessage::parse(&text)))
            }
            Ok(Message::Ping(_)) => {
                // ping 直接转成 WsMessage::Pong 供上层处理
                debug!("Received ping, sending pong");
                Some(Ok(WsMessage::Pong))
            }
            Ok(Message::Pong(_)) => {
                debug!("Received pong");
                None // 不向上游传播
            }
            Ok(Message::Close(_)) => {
                warn!("WebSocket connection closed by server");
                Some(Err(OkxError::ConnectionClosed))
            }
            Ok(Message::Binary(_)) | Ok(Message::Frame(_)) => {
                debug!("Received non-text frame (ignored)");
                None
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                Some(Err(OkxError::WebSocket(e.to_string())))
            }
        }
    }

    /// Login to the private WebSocket.
    ///
    /// Must be called before subscribing to private channels.
    pub async fn login(&mut self) -> Result<()> {
        if !self.is_private {
            return Err(OkxError::Auth(
                "Cannot login on public WebSocket connection".to_string(),
            ));
        }

        if self.is_logged_in {
            debug!("Already logged in");
            return Ok(());
        }

        let signer = Signer::new(self.config.credentials().clone());
        let (api_key, passphrase, timestamp, sign) = signer.generate_ws_login_params();

        let request = WsRequest::login(&api_key, &passphrase, &timestamp, &sign);
        self.send_request(&request).await?;

        // Wait for login response
        while let Some(result) = self.stream.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    let msg = WsMessage::parse(&text);
                    match msg {
                        WsMessage::Event {
                            event: crate::message::WsEvent::Login,
                            code,
                            ..
                        } => {
                            if code.as_deref() == Some("0") {
                                info!("WebSocket login successful");
                                self.is_logged_in = true;
                                return Ok(());
                            } else {
                                let err_msg = format!("Login failed: code={:?}", code);
                                error!("{}", err_msg);
                                return Err(OkxError::Auth(err_msg));
                            }
                        }
                        WsMessage::Event {
                            event: crate::message::WsEvent::Error,
                            code,
                            msg,
                            ..
                        } => {
                            let err_msg = format!(
                                "Login error: code={}, msg={}",
                                code.unwrap_or_default(),
                                msg.unwrap_or_default()
                            );
                            error!("{}", err_msg);
                            return Err(OkxError::Auth(err_msg));
                        }
                        _ => continue,
                    }
                }
                Ok(Message::Ping(data)) => {
                    self.sink
                        .send(Message::Pong(data))
                        .await
                        .map_err(|e| OkxError::WebSocket(e.to_string()))?;
                }
                Err(e) => {
                    return Err(OkxError::WebSocket(e.to_string()));
                }
                _ => continue,
            }
        }

        Err(OkxError::ConnectionClosed)
    }

    /// Subscribe to channels.
    ///
    /// For private channels, login must be called first.
    pub async fn subscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        // Check if any channel is private and we need to login
        let has_private = channels.iter().any(|c| c.is_private());
        if has_private && !self.is_logged_in {
            if self.is_private {
                self.login().await?;
            } else {
                return Err(OkxError::Auth(
                    "Cannot subscribe to private channels on public connection".to_string(),
                ));
            }
        }

        let args: Vec<_> = channels
            .iter()
            .map(|c| serde_json::to_value(c).expect("Channel serialization should not fail"))
            .collect();

        let request = WsRequest::subscribe(args);
        self.send_request(&request).await
    }

    /// Unsubscribe from channels.
    pub async fn unsubscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        let args: Vec<_> = channels
            .iter()
            .map(|c| serde_json::to_value(c).expect("Channel serialization should not fail"))
            .collect();

        let request = WsRequest::unsubscribe(args);
        self.send_request(&request).await
    }

    /// Send a ping to keep the connection alive.
    pub async fn ping(&mut self) -> Result<()> {
        debug!("Sending ping");
        self.sink
            .send(Message::Text("ping".into()))
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))
    }

    /// Close the WebSocket connection.
    pub async fn close(&mut self) -> Result<()> {
        info!("Closing WebSocket connection");
        self.sink
            .close()
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))
    }

    /// Send a request to the WebSocket.
    async fn send_request(&mut self, request: &WsRequest) -> Result<()> {
        let json = serde_json::to_string(request)?;
        debug!("Sending: {}", json);
        self.sink
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))
    }

    /// Process the next message from the WebSocket.
    fn process_message(
        &mut self,
        result: std::result::Result<Message, WsError>,
    ) -> Option<Result<WsMessage>> {
        Self::handle_message(result)
    }
}

impl<S> Stream for WsClientInner<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    type Item = Result<WsMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Check heartbeat timer
        if self.heartbeat.poll_tick(cx).is_ready() {
            // We should send a ping, but we can't do async here
            // The user should call ping() periodically or we handle it differently
        }

        // Poll the WebSocket stream
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Ready(Some(result)) => {
                if let Some(msg) = self.process_message(result) {
                    Poll::Ready(Some(msg))
                } else {
                    // Message was filtered out, poll again
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// OKX WebSocket 客户端。
///
/// 提供公有/私有连接、订阅管理与消息流式读取。
pub struct WsClient {
    inner: WsClientInner<MaybeTlsStream<TcpStream>>,
}

impl WsClient {
    /// 处理 WebSocket 消息并转换为 WsMessage，供流或测试使用。
    pub fn handle_message(
        result: std::result::Result<Message, WsError>,
    ) -> Option<Result<WsMessage>> {
        WsClientInner::<MaybeTlsStream<TcpStream>>::handle_message(result)
    }

    /// Connect to the public WebSocket endpoint.
    ///
    /// Public channels do not require authentication.
    pub async fn connect_public(config: &Config) -> Result<Self> {
        let url = config.ws_public_url();
        info!("Connecting to public WebSocket: {}", url);

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))?;

        Ok(Self {
            inner: WsClientInner::new(ws_stream, config.clone(), false),
        })
    }

    /// Connect to the private WebSocket endpoint.
    ///
    /// Private channels require authentication via login.
    pub async fn connect_private(config: &Config) -> Result<Self> {
        let url = config.ws_private_url();
        info!("Connecting to private WebSocket: {}", url);

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))?;

        Ok(Self {
            inner: WsClientInner::new(ws_stream, config.clone(), true),
        })
    }

    /// Login to the private WebSocket.
    ///
    /// Must be called before subscribing to private channels.
    pub async fn login(&mut self) -> Result<()> {
        self.inner.login().await
    }

    /// Subscribe to channels.
    ///
    /// For private channels, login must be called first.
    pub async fn subscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        self.inner.subscribe(channels).await
    }

    /// Unsubscribe from channels.
    pub async fn unsubscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        self.inner.unsubscribe(channels).await
    }

    /// Send a ping to keep the connection alive.
    pub async fn ping(&mut self) -> Result<()> {
        self.inner.ping().await
    }

    /// Close the WebSocket connection.
    pub async fn close(&mut self) -> Result<()> {
        self.inner.close().await
    }
}

impl Stream for WsClient {
    type Item = Result<WsMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::{SinkExt, StreamExt};
    use okx_core::Credentials;
    use tokio::io::{duplex, DuplexStream};
    use tokio::time::timeout;
    use tokio_tungstenite::{accept_async, client_async};

    async fn in_memory_client(
        is_private: bool,
    ) -> (WsClientInner<DuplexStream>, WebSocketStream<DuplexStream>) {
        let (client_io, server_io) = duplex(1024);

        let (client_res, server_res) = tokio::join!(
            client_async("ws://localhost/ws", client_io),
            accept_async(server_io)
        );

        let (client_ws, _) = client_res.expect("in-memory client handshake 失败");
        let server_ws = server_res.expect("in-memory server handshake 失败");

        let cfg = Config::new(Credentials::new("k", "s", "p")).simulated(true);
        (WsClientInner::new(client_ws, cfg, is_private), server_ws)
    }

    #[tokio::test]
    async fn private_login_success_handles_ping_and_is_idempotent() {
        let (mut client, mut server) = in_memory_client(true).await;

        let server_task = tokio::spawn(async move {
            let msg = server
                .next()
                .await
                .expect("应收到登录请求")
                .expect("消息应为 Ok");
            let Message::Text(text) = msg else {
                panic!("预期 Text 登录请求，实际为: {msg:?}");
            };
            let v: serde_json::Value = serde_json::from_str(&text).expect("登录请求应为 JSON");
            assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("login"));

            let _ = server.send(Message::Ping(vec![1, 2, 3].into())).await;
            let _ = server.next().await;
            let _ = server
                .send(Message::Text(r#"{"event":"subscribe"}"#.into()))
                .await;
            let _ = server
                .send(Message::Text(
                    r#"{"event":"login","code":"0","msg":"","connId":"local"}"#.into(),
                ))
                .await;

            let _ = server.close(None).await;
        });

        client.login().await.expect("首次登录应成功");
        client.login().await.expect("重复登录应直接返回 Ok");

        server_task.await.expect("服务端任务不应 panic");
    }

    #[tokio::test]
    async fn private_login_failure_returns_auth_error() {
        let (mut client, mut server) = in_memory_client(true).await;

        let server_task = tokio::spawn(async move {
            let _ = server.next().await;
            let _ = server
                .send(Message::Text(
                    r#"{"event":"login","code":"60001","msg":"login failed"}"#.into(),
                ))
                .await;
        });

        let err = client.login().await.expect_err("登录应失败");
        assert!(matches!(err, OkxError::Auth(_)));

        server_task.await.expect("服务端任务不应 panic");
    }

    #[tokio::test]
    async fn private_login_error_event_returns_auth_error() {
        let (mut client, mut server) = in_memory_client(true).await;

        let server_task = tokio::spawn(async move {
            let _ = server.next().await;
            let _ = server
                .send(Message::Text(
                    r#"{"event":"error","code":"60001","msg":"invalid request"}"#.into(),
                ))
                .await;
        });

        let err = client.login().await.expect_err("登录应返回错误");
        assert!(matches!(err, OkxError::Auth(_)));

        server_task.await.expect("服务端任务不应 panic");
    }

    #[tokio::test]
    async fn private_login_returns_connection_closed_when_server_closes() {
        let (mut client, mut server) = in_memory_client(true).await;

        let server_task = tokio::spawn(async move {
            let _ = server.next().await;
            let _ = server.close(None).await;
        });

        let err = client.login().await.expect_err("应收到连接关闭错误");
        assert!(matches!(
            err,
            OkxError::ConnectionClosed | OkxError::WebSocket(_)
        ));

        server_task.await.expect("服务端任务不应 panic");
    }

    #[tokio::test]
    async fn subscribing_private_channel_on_public_connection_is_rejected() {
        let (mut client, _server) = in_memory_client(false).await;
        let err = client
            .subscribe(vec![Channel::Account { ccy: None }])
            .await
            .expect_err("公有连接不应允许订阅私有频道");
        assert!(matches!(err, OkxError::Auth(_)));
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_and_poll_next_filters_pong() {
        let (mut client, mut server) = in_memory_client(false).await;

        let server_task = tokio::spawn(async move {
            // subscribe
            let msg = server
                .next()
                .await
                .expect("应收到 subscribe 请求")
                .expect("消息应为 Ok");
            let Message::Text(text) = msg else {
                panic!("预期 Text subscribe 请求，实际为: {msg:?}");
            };
            let v: serde_json::Value =
                serde_json::from_str(&text).expect("subscribe 请求应为 JSON");
            assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("subscribe"));

            let _ = server.send(Message::Pong(vec![9].into())).await;
            let _ = server
                .send(Message::Text(r#"{"event":"subscribe"}"#.into()))
                .await;

            // unsubscribe
            let msg = server
                .next()
                .await
                .expect("应收到 unsubscribe 请求")
                .expect("消息应为 Ok");
            let Message::Text(text) = msg else {
                panic!("预期 Text unsubscribe 请求，实际为: {msg:?}");
            };
            let v: serde_json::Value =
                serde_json::from_str(&text).expect("unsubscribe 请求应为 JSON");
            assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("unsubscribe"));

            let _ = server
                .send(Message::Text(r#"{"event":"unsubscribe"}"#.into()))
                .await;
        });

        client
            .subscribe(vec![Channel::Tickers {
                inst_id: "BTC-USDT".to_string(),
            }])
            .await
            .expect("订阅应成功发送");

        let msg = timeout(Duration::from_secs(1), client.next())
            .await
            .expect("等待 subscribe 事件超时")
            .expect("应收到消息")
            .expect("消息应为 Ok");
        match msg {
            WsMessage::Event { event, .. } => assert_eq!(event, crate::message::WsEvent::Subscribe),
            other => panic!("预期 subscribe 事件，实际为: {other:?}"),
        }

        client
            .unsubscribe(vec![Channel::Tickers {
                inst_id: "BTC-USDT".to_string(),
            }])
            .await
            .expect("取消订阅应成功发送");

        let msg = timeout(Duration::from_secs(1), client.next())
            .await
            .expect("等待 unsubscribe 事件超时")
            .expect("应收到消息")
            .expect("消息应为 Ok");
        match msg {
            WsMessage::Event { event, .. } => {
                assert_eq!(event, crate::message::WsEvent::Unsubscribe)
            }
            other => panic!("预期 unsubscribe 事件，实际为: {other:?}"),
        }

        server_task.await.expect("服务端任务不应 panic");
    }

    #[tokio::test]
    async fn ping_sends_text_ping() {
        let (mut client, mut server) = in_memory_client(false).await;

        client.ping().await.expect("ping 发送失败");

        let msg = timeout(Duration::from_secs(1), server.next())
            .await
            .expect("等待 ping 超时")
            .expect("应收到消息")
            .expect("消息应为 Ok");

        let Message::Text(text) = msg else {
            panic!("预期 Text ping，实际为: {msg:?}");
        };
        assert_eq!(text, "ping");
    }
}
