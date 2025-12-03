use std::env;

use okx_core::{Config, Credentials};
use okx_rest::{AccountApi, MarketApi, OkxRestClient};

/// 基础 REST 示例：公共行情 + 可选私有账户调用（默认模拟盘）。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥，未提供则使用占位值并跳过私有接口
    let api_key = env::var("OKX_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let secret_key = env::var("OKX_SECRET_KEY").unwrap_or_else(|_| "your_secret_key".to_string());
    let passphrase = env::var("OKX_PASSPHRASE").unwrap_or_else(|_| "your_passphrase".to_string());
    let has_real_key = api_key != "your_api_key";

    let creds = Credentials::new(api_key, secret_key, passphrase);
    let config = Config::new(creds).simulated(true).with_timeout_secs(10);
    let client = OkxRestClient::new(config);

    // 公共接口：单个 ticker
    let ticker = client.get_ticker("BTC-USDT").await?;
    if let Some(t) = ticker.first() {
        println!("BTC-USDT last={} bid={} ask={}", t.last, t.bid_px, t.ask_px);
    }

    // 公共接口：全部 SPOT ticker（仅展示数量）
    let spot_tickers = client
        .get_tickers(okx_rest::api::market::GetTickersParams {
            inst_type: "SPOT".to_string(),
            uly: None,
            inst_family: None,
        })
        .await?;
    println!("Loaded {} SPOT tickers", spot_tickers.len());

    // 私有接口：需要真实密钥
    if has_real_key {
        let balances = client.get_balance(None).await?;
        let total_eq = balances
            .get(0)
            .map(|b| b.total_eq.clone())
            .unwrap_or_else(|| "0".to_string());
        println!("Total equity: {total_eq}");
    } else {
        println!("Skip private calls (set OKX_API_KEY/OKX_SECRET_KEY/OKX_PASSPHRASE to enable)");
    }

    Ok(())
}
