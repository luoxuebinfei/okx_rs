use okx_core::{Config, Credentials};
use okx_rest::{MarketApi, OkxRestClient};

/// 现货价格示例：查询单个现货 ticker 并打印基础行情。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 公共接口无需真实密钥，这里用占位值构建客户端
    let config = Config::new(Credentials::new(
        "demo_api_key",
        "demo_secret_key",
        "demo_passphrase",
    ))
    .with_timeout_secs(10);
    let client = OkxRestClient::new(config);

    let inst_id = "BTC-USDT";
    let tickers = client.get_ticker(inst_id).await?;

    if let Some(t) = tickers.first() {
        println!(
            "{inst_id} 最新价={} 买一={} 卖一={} 24h量={} 时间戳={}",
            t.last, t.bid_px, t.ask_px, t.vol_24h, t.ts
        );
    } else {
        println!("未返回 {inst_id} 行情数据");
    }

    Ok(())
}
