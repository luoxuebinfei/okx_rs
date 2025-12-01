use std::time::Duration;

use futures_util::StreamExt;
use okx_core::{Config, Credentials};
use okx_ws::{Channel, WsClient, WsMessage};

/// 公共 WS 示例：订阅 tickers 与 trades，并读取前若干条消息。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inst_id = "BTC-USDT";
    let creds = Credentials::new("api_key", "secret_key", "passphrase");
    let config = Config::new(creds).simulated(true);
    let mut client = WsClient::connect_public(&config).await?;

    client
        .subscribe(vec![
            Channel::Tickers {
                inst_id: inst_id.into(),
            },
            Channel::Trades {
                inst_id: inst_id.into(),
            },
        ])
        .await?;

    let mut seen = 0;
    while let Some(msg) = client.next().await {
        match msg? {
            WsMessage::Data { channel, data, .. } => {
                println!("channel={channel} data={data:?}");
                seen += 1;
                if seen >= 5 {
                    break;
                }
            }
            WsMessage::Event { event, code, msg, .. } => {
                println!("event={event:?} code={code:?} msg={msg:?}");
            }
            _ => {}
        }
    }

    // 发送一次 ping 保持连接（可选）
    client.ping().await?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    client.close().await?;

    Ok(())
}
