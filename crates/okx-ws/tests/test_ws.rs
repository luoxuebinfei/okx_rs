//! Integration tests for okx-ws.
//!
//! These tests verify WebSocket functionality including:
//! - Connection management
//! - Channel subscriptions
//! - Message parsing
//! - Reconnection logic

use std::time::Duration;

use okx_core::{Config, Credentials};
use okx_ws::{
    Channel, ConnectionState, ConnectionType, ReconnectConfig, ReconnectingWsClient, WsClient,
    WsEvent, WsMessage,
};

/// Create a test config with dummy credentials.
fn create_test_config() -> Config {
    let creds = Credentials::new("dummy_key", "dummy_secret", "dummy_passphrase");
    Config::new(creds).simulated(true)
}

mod channel_tests {
    use super::*;

    #[test]
    fn test_channel_is_private() {
        // Public channels
        assert!(!Channel::Tickers {
            inst_id: "BTC-USDT".to_string()
        }
        .is_private());
        assert!(!Channel::Books {
            inst_id: "BTC-USDT".to_string()
        }
        .is_private());
        assert!(!Channel::Trades {
            inst_id: "BTC-USDT".to_string()
        }
        .is_private());

        // Private channels
        assert!(Channel::Account { ccy: None }.is_private());
        assert!(Channel::Positions {
            inst_type: "SWAP".to_string(),
            inst_family: None,
            inst_id: None
        }
        .is_private());
        assert!(Channel::Orders {
            inst_type: "SPOT".to_string(),
            inst_family: None,
            inst_id: None
        }
        .is_private());
        assert!(Channel::BalanceAndPosition.is_private());
    }

    #[test]
    fn test_channel_name() {
        assert_eq!(
            Channel::Tickers {
                inst_id: "BTC-USDT".to_string()
            }
            .name(),
            "tickers"
        );
        assert_eq!(
            Channel::Books {
                inst_id: "BTC-USDT".to_string()
            }
            .name(),
            "books"
        );
        assert_eq!(
            Channel::Trades {
                inst_id: "BTC-USDT".to_string()
            }
            .name(),
            "trades"
        );
        assert_eq!(Channel::Account { ccy: None }.name(), "account");
        assert_eq!(
            Channel::Positions {
                inst_type: "SWAP".to_string(),
                inst_family: None,
                inst_id: None
            }
            .name(),
            "positions"
        );
    }

    #[test]
    fn test_channel_serialization() {
        let channel = Channel::Tickers {
            inst_id: "BTC-USDT".to_string(),
        };
        let json = serde_json::to_string(&channel).unwrap();
        assert!(json.contains("tickers"));
        assert!(json.contains("BTC-USDT"));
    }

    #[test]
    fn test_channel_deserialization() {
        let json = r#"{"channel":"tickers","instId":"ETH-USDT"}"#;
        let channel: Channel = serde_json::from_str(json).unwrap();
        match channel {
            Channel::Tickers { inst_id } => assert_eq!(inst_id, "ETH-USDT"),
            _ => panic!("Wrong channel type"),
        }
    }
}

mod message_tests {
    use super::*;

    #[test]
    fn test_parse_pong() {
        let msg = WsMessage::parse("pong");
        assert!(matches!(msg, WsMessage::Pong));
    }

    #[test]
    fn test_parse_subscribe_event() {
        let json = r#"{"event":"subscribe","arg":{"channel":"tickers","instId":"BTC-USDT"}}"#;
        let msg = WsMessage::parse(json);
        match msg {
            WsMessage::Event { event, arg, .. } => {
                assert_eq!(event, WsEvent::Subscribe);
                assert!(arg.is_some());
            }
            _ => panic!("Expected Event message"),
        }
    }

    #[test]
    fn test_parse_login_event() {
        let json = r#"{"event":"login","code":"0","msg":"","connId":"abc123"}"#;
        let msg = WsMessage::parse(json);
        match msg {
            WsMessage::Event {
                event,
                code,
                conn_id,
                ..
            } => {
                assert_eq!(event, WsEvent::Login);
                assert_eq!(code, Some("0".to_string()));
                assert_eq!(conn_id, Some("abc123".to_string()));
            }
            _ => panic!("Expected Event message"),
        }
    }

    #[test]
    fn test_parse_error_event() {
        let json = r#"{"event":"error","code":"60001","msg":"Invalid request"}"#;
        let msg = WsMessage::parse(json);
        match msg {
            WsMessage::Event {
                event, code, msg, ..
            } => {
                assert_eq!(event, WsEvent::Error);
                assert_eq!(code, Some("60001".to_string()));
                assert_eq!(msg, Some("Invalid request".to_string()));
            }
            _ => panic!("Expected Event message"),
        }
    }

    #[test]
    fn test_parse_data_message() {
        let json = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instId":"BTC-USDT","last":"50000"}]}"#;
        let msg = WsMessage::parse(json);
        match msg {
            WsMessage::Data { channel, data, .. } => {
                assert_eq!(channel, "tickers");
                assert!(!data.is_empty());
            }
            _ => panic!("Expected Data message"),
        }
    }

    #[test]
    fn test_parse_unknown() {
        let msg = WsMessage::parse("invalid json {{{");
        assert!(matches!(msg, WsMessage::Unknown(_)));
    }

    #[test]
    fn test_parse_unknown_event() {
        let json = r#"{"event":"other","arg":{}}"#;
        let msg = WsMessage::parse(json);
        assert!(matches!(msg, WsMessage::Unknown(_)));
    }

    #[test]
    fn test_is_error() {
        let error_msg = WsMessage::Event {
            event: WsEvent::Error,
            arg: None,
            code: Some("60001".to_string()),
            msg: Some("Error".to_string()),
            conn_id: None,
        };
        assert!(error_msg.is_error());

        let ok_msg = WsMessage::Event {
            event: WsEvent::Subscribe,
            arg: None,
            code: None,
            msg: None,
            conn_id: None,
        };
        assert!(!ok_msg.is_error());
    }

    #[test]
    fn test_error_details() {
        let msg = WsMessage::Event {
            event: WsEvent::Error,
            arg: None,
            code: Some("60001".to_string()),
            msg: Some("Invalid request".to_string()),
            conn_id: None,
        };

        let details = msg.error_details();
        assert!(details.is_some());
        let (code, message) = details.unwrap();
        assert_eq!(code, "60001");
        assert_eq!(message, "Invalid request");
    }

    #[test]
    fn test_parse_data_single_object() {
        let json = r#"{"arg":{"channel":"books","instId":"BTC-USDT"},"data":{"px":"1","sz":"1"}}"#;
        let msg = WsMessage::parse(json);
        match msg {
            WsMessage::Data { channel, data, .. } => {
                assert_eq!(channel, "books");
                assert_eq!(data.len(), 1);
            }
            _ => panic!("应解析为 Data"),
        }
    }

    #[test]
    fn test_error_details_none_on_non_error() {
        let msg = WsMessage::Event {
            event: WsEvent::Subscribe,
            arg: None,
            code: None,
            msg: None,
            conn_id: None,
        };
        assert!(msg.error_details().is_none());
    }

    #[test]
    fn test_wsevent_display() {
        let cases = vec![
            (WsEvent::Subscribe, "subscribe"),
            (WsEvent::Unsubscribe, "unsubscribe"),
            (WsEvent::Login, "login"),
            (WsEvent::Error, "error"),
        ];
        for (event, expected) in cases {
            assert_eq!(event.to_string(), expected);
        }
    }
}

mod reconnect_config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ReconnectConfig::default();
        assert_eq!(config.initial_delay, Duration::from_secs(1));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.max_attempts.is_none());
        assert!(config.restore_subscriptions);
    }

    #[test]
    fn test_config_builder() {
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
}

mod connection_tests {
    use super::*;
    use futures_util::StreamExt;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_connect_public() {
        let config = create_test_config();

        match timeout(Duration::from_secs(10), WsClient::connect_public(&config)).await {
            Ok(Ok(_client)) => {
                // Connection successful
                // Client is connected
            }
            Ok(Err(e)) => {
                // Connection error (network issues in CI)
                eprintln!("Connection error (acceptable): {}", e);
            }
            Err(_) => {
                eprintln!("Connection timeout (acceptable in CI)");
            }
        }
    }

    #[tokio::test]
    async fn test_reconnecting_client_connect() {
        let config = create_test_config();
        let reconnect_config = ReconnectConfig::default().with_max_attempts(3);

        match timeout(
            Duration::from_secs(10),
            ReconnectingWsClient::connect(config, ConnectionType::Public, reconnect_config),
        )
        .await
        {
            Ok(Ok(client)) => {
                assert_eq!(client.state(), ConnectionState::Connected);
                assert!(client.is_connected());
                assert_eq!(client.subscription_count(), 0);
            }
            Ok(Err(e)) => {
                eprintln!("Connection error (acceptable): {}", e);
            }
            Err(_) => {
                eprintln!("Connection timeout (acceptable in CI)");
            }
        }
    }

    #[tokio::test]
    async fn test_subscribe_and_receive() {
        let config = create_test_config();

        let result = timeout(Duration::from_secs(10), WsClient::connect_public(&config)).await;

        if let Ok(Ok(mut client)) = result {
            // Subscribe to ticker
            let sub_result = client
                .subscribe(vec![Channel::Tickers {
                    inst_id: "BTC-USDT".to_string(),
                }])
                .await;

            if sub_result.is_ok() {
                // Try to receive a message
                let msg_result = timeout(Duration::from_secs(5), client.next()).await;

                match msg_result {
                    Ok(Some(Ok(msg))) => {
                        // Got a message
                        match msg {
                            WsMessage::Event { event, .. } => {
                                assert!(matches!(event, WsEvent::Subscribe));
                            }
                            WsMessage::Data { channel, .. } => {
                                assert_eq!(channel, "tickers");
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        eprintln!("No message received (acceptable in CI)");
                    }
                }
            }
        }
    }
}
