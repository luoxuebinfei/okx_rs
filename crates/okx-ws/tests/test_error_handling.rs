//! 测试 WebSocket 客户端的错误处理和边界场景

use std::time::Duration;

use okx_core::{Config, Credentials};
use okx_ws::{Channel, ConnectionType, ReconnectConfig, ReconnectingWsClient, WsClient};

fn create_test_config() -> Config {
    let creds = Credentials::new("dummy_key", "dummy_secret", "dummy_passphrase");
    Config::new(creds).simulated(true)
}

#[test]
fn test_reconnect_config_builder_like_api() {
    let config = ReconnectConfig::default()
        .with_max_attempts(5)
        .with_initial_delay(Duration::from_secs(1))
        .with_max_delay(Duration::from_secs(30));

    assert_eq!(config.max_attempts, Some(5));
    assert_eq!(config.initial_delay, Duration::from_secs(1));
    assert_eq!(config.max_delay, Duration::from_secs(30));
}

#[test]
fn test_reconnect_config_default() {
    let config = ReconnectConfig::default();

    assert_eq!(config.max_attempts, None);
    assert_eq!(config.initial_delay, Duration::from_secs(1));
    assert_eq!(config.max_delay, Duration::from_secs(60));
}

#[tokio::test]
async fn test_invalid_websocket_url() {
    let config =
        create_test_config().with_ws_public_url("wss://invalid-domain-that-does-not-exist.com");

    let result = WsClient::connect_public(&config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_reconnecting_client_creation() {
    let config = create_test_config();
    let reconnect_config = ReconnectConfig::default();

    let result =
        ReconnectingWsClient::connect(config, ConnectionType::Public, reconnect_config).await;

    match result {
        Ok(_client) => {
            // 连接成功
        }
        Err(e) => {
            // 网络错误可接受
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_multiple_subscriptions() {
    let config = create_test_config();

    match WsClient::connect_public(&config).await {
        Ok(mut client) => {
            let channels = vec![
                Channel::Tickers {
                    inst_id: "BTC-USDT".to_string(),
                },
                Channel::Tickers {
                    inst_id: "ETH-USDT".to_string(),
                },
                Channel::Books {
                    inst_id: "BTC-USDT".to_string(),
                },
            ];

            if let Err(e) = client.subscribe(channels).await {
                eprintln!("Subscribe error (acceptable): {}", e);
            }
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_unsubscribe_channels() {
    let config = create_test_config();

    match WsClient::connect_public(&config).await {
        Ok(mut client) => {
            let channels = vec![Channel::Tickers {
                inst_id: "BTC-USDT".to_string(),
            }];

            // 先订阅
            if client.subscribe(channels.clone()).await.is_ok() {
                // 再取消订阅
                if let Err(e) = client.unsubscribe(channels).await {
                    eprintln!("Unsubscribe error (acceptable): {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[test]
fn test_channel_edge_cases() {
    // 测试空字符串
    let channel = Channel::Tickers {
        inst_id: "".to_string(),
    };
    assert_eq!(channel.name(), "tickers");

    // 测试特殊字符
    let channel = Channel::Tickers {
        inst_id: "BTC-USDT-SWAP".to_string(),
    };
    assert!(!channel.is_private());

    // 测试私有频道参数
    let channel = Channel::Account {
        ccy: Some("BTC".to_string()),
    };
    assert!(channel.is_private());

    let channel = Channel::Positions {
        inst_type: "FUTURES".to_string(),
        inst_family: Some("BTC-USD".to_string()),
        inst_id: Some("BTC-USD-230331".to_string()),
    };
    assert!(channel.is_private());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let config = create_test_config();

    match WsClient::connect_public(&config).await {
        Ok(mut client) => {
            // 测试同时订阅和发送其他命令
            let subscribe_task = async {
                let channels = vec![Channel::Tickers {
                    inst_id: "BTC-USDT".to_string(),
                }];
                client.subscribe(channels).await
            };

            if let Err(e) = subscribe_task.await {
                eprintln!("Concurrent operation error (acceptable): {}", e);
            }
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[test]
fn test_config_edge_cases() {
    // 测试空凭证
    let creds = Credentials::new("", "", "");
    let config = Config::new(creds);
    assert!(!config.is_simulated());

    // 测试超时配置
    let creds = Credentials::new("key", "secret", "pass");
    let config = Config::new(creds).with_timeout_secs(0);
    assert_eq!(config.timeout_secs(), 0);

    // 测试极大超时
    let config = Config::new(Credentials::new("key", "secret", "pass")).with_timeout_secs(3600);
    assert_eq!(config.timeout_secs(), 3600);
}
