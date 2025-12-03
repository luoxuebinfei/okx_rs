use futures_util::StreamExt;
use okx_core::{Config, Credentials};
use okx_ws::{Channel, WsClient, WsMessage};

/// 永续合约 WS 示例：订阅指定合约的 ticker、成交与资金费率，按 Ctrl+C 手动退出。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inst_id = "BTC-USDT-SWAP";

    // 公共频道无需真实密钥，这里使用占位值；如需实盘可去掉 .simulated(true)
    let config = Config::new(Credentials::new(
        "demo_api_key",
        "demo_secret_key",
        "demo_passphrase",
    ))
    .simulated(true);
    let mut client = WsClient::connect_public(&config).await?;

    client
        .subscribe(vec![
            Channel::Tickers {
                inst_id: inst_id.into(),
            },
            Channel::Trades {
                inst_id: inst_id.into(),
            },
            Channel::FundingRate {
                inst_id: inst_id.into(),
            },
        ])
        .await?;

    println!("已订阅 {inst_id}，按 Ctrl+C 手动退出...");

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                println!("收到退出信号，关闭连接...");
                break;
            }
            msg = client.next() => {
                match msg {
                    Some(Ok(WsMessage::Data { channel, data, .. })) => {
                        println!("channel={channel} data={data:?}");
                    }
                    Some(Ok(WsMessage::Event { event, code, msg, .. })) => {
                        println!("event={event:?} code={code:?} msg={msg:?}");
                    }
                    Some(Ok(_)) => {}
                    Some(Err(e)) => {
                        eprintln!("WS error: {e}");
                    }
                    None => {
                        eprintln!("流已结束，准备退出");
                        break;
                    }
                }
            }
        }
    }

    client.close().await?;
    Ok(())
}
