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

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// WebSocket client for OKX exchange.
///
/// Provides streaming access to real-time market data and account updates.
pub struct WsClient {
    /// WebSocket write half
    sink: SplitSink<WsStream, Message>,
    /// WebSocket read half
    stream: SplitStream<WsStream>,
    /// Client configuration
    config: Config,
    /// Whether this is a private connection
    is_private: bool,
    /// Whether logged in (for private connections)
    is_logged_in: bool,
    /// Heartbeat interval
    heartbeat: Interval,
}

impl WsClient {
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

    /// Connect to the public WebSocket endpoint.
    ///
    /// Public channels do not require authentication.
    pub async fn connect_public(config: &Config) -> Result<Self> {
        let url = config.ws_public_url();
        info!("Connecting to public WebSocket: {}", url);

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| OkxError::WebSocket(e.to_string()))?;

        let (sink, stream) = ws_stream.split();

        Ok(Self {
            sink,
            stream,
            config: config.clone(),
            is_private: false,
            is_logged_in: false,
            heartbeat: interval(Duration::from_secs(25)),
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

        let (sink, stream) = ws_stream.split();

        Ok(Self {
            sink,
            stream,
            config: config.clone(),
            is_private: true,
            is_logged_in: false,
            heartbeat: interval(Duration::from_secs(25)),
        })
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

impl Stream for WsClient {
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
