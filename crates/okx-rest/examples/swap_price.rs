use okx_core::{Config, Credentials};
use okx_rest::{api::market::GetTickersParams, MarketApi, OkxRestClient};

/// 永续合约价格示例：按 instType=SWAP 获取行情并筛选指定合约。
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

    let inst_id = "BTC-USDT-SWAP";
    let tickers = client
        .get_tickers(GetTickersParams {
            inst_type: "SWAP".into(),
            uly: Some("BTC-USDT".into()),
            inst_family: None,
        })
        .await?;

    if let Some(t) = tickers.iter().find(|t| t.inst_id == inst_id) {
        println!(
            "{inst_id} 最新价={} 买一={} 卖一={} 24h量={} 时间戳={}",
            t.last, t.bid_px, t.ask_px, t.vol_24h, t.ts
        );
    } else {
        println!("未找到 {inst_id} 行情，请确认合约代码或是否交易中");
    }

    Ok(())
}
