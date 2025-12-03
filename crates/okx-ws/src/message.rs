//! WebSocket message types.
//!
//! Source: OKX API v5 WebSocket API
//! - <https://www.okx.com/docs-v5/en/#websocket-api>

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// WebSocket message received from OKX.
#[derive(Debug, Clone)]
pub enum WsMessage {
    /// Data push message
    Data {
        /// Channel name
        channel: String,
        /// Argument that was subscribed
        arg: Value,
        /// Data payload
        data: Vec<Value>,
    },
    /// Event message (subscribe/unsubscribe/login response)
    Event {
        /// Event type
        event: WsEvent,
        /// Argument
        arg: Option<Value>,
        /// Response code (for errors)
        code: Option<String>,
        /// Response message
        msg: Option<String>,
        /// Connection ID (for login)
        conn_id: Option<String>,
    },
    /// Pong response
    Pong,
    /// Unknown message
    Unknown(String),
}

/// WebSocket event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsEvent {
    /// Subscription confirmed
    Subscribe,
    /// Unsubscription confirmed
    Unsubscribe,
    /// Login successful
    Login,
    /// Error occurred
    Error,
}

impl std::fmt::Display for WsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Subscribe => "subscribe",
            Self::Unsubscribe => "unsubscribe",
            Self::Login => "login",
            Self::Error => "error",
        };
        f.write_str(s)
    }
}

impl WsMessage {
    /// Parse a WebSocket message from JSON text.
    pub fn parse(text: &str) -> Self {
        // Handle pong response
        if text == "pong" {
            return Self::Pong;
        }

        // Try to parse as JSON
        let value: Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(_) => return Self::Unknown(text.to_string()),
        };

        // Check if it's an event message
        if let Some(event) = value.get("event").and_then(|e| e.as_str()) {
            let ws_event = match event {
                "subscribe" => WsEvent::Subscribe,
                "unsubscribe" => WsEvent::Unsubscribe,
                "login" => WsEvent::Login,
                "error" => WsEvent::Error,
                _ => return Self::Unknown(text.to_string()),
            };

            return Self::Event {
                event: ws_event,
                arg: value.get("arg").cloned(),
                code: value.get("code").and_then(|c| c.as_str()).map(String::from),
                msg: value.get("msg").and_then(|m| m.as_str()).map(String::from),
                conn_id: value
                    .get("connId")
                    .and_then(|c| c.as_str())
                    .map(String::from),
            };
        }

        // Check if it's a data message
        if let (Some(arg), Some(data)) = (value.get("arg"), value.get("data")) {
            let channel = arg
                .get("channel")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown")
                .to_string();

            let data_vec = match data {
                Value::Array(arr) => arr.clone(),
                _ => vec![data.clone()],
            };

            return Self::Data {
                channel,
                arg: arg.clone(),
                data: data_vec,
            };
        }

        Self::Unknown(text.to_string())
    }

    /// Check if this is an error event.
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            Self::Event {
                event: WsEvent::Error,
                ..
            }
        )
    }

    /// Get error details if this is an error event.
    #[must_use]
    pub fn error_details(&self) -> Option<(&str, &str)> {
        match self {
            Self::Event {
                event: WsEvent::Error,
                code,
                msg,
                ..
            } => {
                let code = code.as_deref().unwrap_or("unknown");
                let msg = msg.as_deref().unwrap_or("unknown error");
                Some((code, msg))
            }
            _ => None,
        }
    }
}

/// WebSocket request message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsRequest {
    /// Operation type
    pub op: String,
    /// Arguments
    pub args: Vec<Value>,
}

impl WsRequest {
    /// Create a subscribe request.
    pub fn subscribe(channels: Vec<Value>) -> Self {
        Self {
            op: "subscribe".to_string(),
            args: channels,
        }
    }

    /// Create an unsubscribe request.
    pub fn unsubscribe(channels: Vec<Value>) -> Self {
        Self {
            op: "unsubscribe".to_string(),
            args: channels,
        }
    }

    /// Create a login request.
    pub fn login(api_key: &str, passphrase: &str, timestamp: &str, sign: &str) -> Self {
        let arg = serde_json::json!({
            "apiKey": api_key,
            "passphrase": passphrase,
            "timestamp": timestamp,
            "sign": sign
        });
        Self {
            op: "login".to_string(),
            args: vec![arg],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ws_request_builders_preserve_payloads() {
        let channels = vec![json!({"channel":"tickers","instId":"BTC-USDT"})];
        let sub = WsRequest::subscribe(channels.clone());
        assert_eq!(sub.op, "subscribe");
        assert_eq!(sub.args, channels);

        let unsub = WsRequest::unsubscribe(channels.clone());
        assert_eq!(unsub.op, "unsubscribe");
        assert_eq!(unsub.args, channels);

        let login = WsRequest::login("api", "pass", "123", "sig");
        assert_eq!(login.op, "login");
        assert_eq!(login.args.len(), 1);
        let arg = login.args[0].as_object().expect("login arg object");
        assert_eq!(arg.get("apiKey").and_then(|v| v.as_str()), Some("api"));
        assert_eq!(arg.get("passphrase").and_then(|v| v.as_str()), Some("pass"));
        assert_eq!(arg.get("timestamp").and_then(|v| v.as_str()), Some("123"));
        assert_eq!(arg.get("sign").and_then(|v| v.as_str()), Some("sig"));
    }

    #[test]
    fn parse_handles_data_array_and_single_object() {
        let data_json = json!({
            "arg": {"channel": "tickers", "instId": "BTC-USDT"},
            "data": [
                {"last": "1.1"},
                {"last": "1.2"}
            ]
        })
        .to_string();

        if let WsMessage::Data { channel, arg, data } = WsMessage::parse(&data_json) {
            assert_eq!(channel, "tickers");
            assert_eq!(arg["instId"], "BTC-USDT");
            assert_eq!(data.len(), 2);
        } else {
            panic!("应解析为 Data");
        }

        let single_json = json!({
            "arg": {"channel": "tickers", "instId": "ETH-USDT"},
            "data": {"last": "2.0"}
        })
        .to_string();

        if let WsMessage::Data { channel, arg, data } = WsMessage::parse(&single_json) {
            assert_eq!(channel, "tickers");
            assert_eq!(arg["instId"], "ETH-USDT");
            assert_eq!(data.len(), 1);
            assert_eq!(data[0]["last"], "2.0");
        } else {
            panic!("应解析单对象为 Data");
        }
    }

    #[test]
    fn parse_handles_event_and_error_details() {
        let ok_event = r#"{"event":"subscribe","arg":{"channel":"orders"}}"#;
        if let WsMessage::Event {
            event,
            arg,
            code,
            msg,
            ..
        } = WsMessage::parse(ok_event)
        {
            assert_eq!(event, WsEvent::Subscribe);
            assert_eq!(arg.unwrap()["channel"], "orders");
            assert!(code.is_none());
            assert!(msg.is_none());
        } else {
            panic!("应解析为 Event 订阅确认");
        }

        let err_event = r#"{"event":"error","code":"51000","msg":"invalid","arg":{"foo":"bar"}}"#;
        let parsed = WsMessage::parse(err_event);
        assert!(parsed.is_error());
        let details = parsed.error_details().expect("应有错误详情");
        assert_eq!(details.0, "51000");
        assert_eq!(details.1, "invalid");
    }

    #[test]
    fn parse_pong_and_unknown_paths() {
        assert!(matches!(WsMessage::parse("pong"), WsMessage::Pong));

        // 非 JSON 串走 Unknown
        let msg = WsMessage::parse("not json");
        if let WsMessage::Unknown(raw) = msg {
            assert_eq!(raw, "not json");
        } else {
            panic!("应解析为 Unknown");
        }

        // 未知 event 也走 Unknown
        let msg = WsMessage::parse(r#"{"event":"weird"}"#);
        assert!(matches!(msg, WsMessage::Unknown(_)));
    }
}
