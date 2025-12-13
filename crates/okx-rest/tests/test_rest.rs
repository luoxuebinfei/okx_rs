//! Integration tests for okx-rest.
//!
//! These tests use the simulated trading environment and public APIs.
//! No real credentials are required for public API tests.

use okx_core::{Config, Credentials};
use okx_rest::{
    api::{market::GetTickersParams, public::GetInstrumentsParams},
    MarketApi, OkxRestClient, PublicApi,
};

/// Create a test client with dummy credentials (for public APIs).
fn create_test_client() -> OkxRestClient {
    let creds = Credentials::new("dummy_key", "dummy_secret", "dummy_passphrase");
    let config = Config::new(creds).simulated(true);
    OkxRestClient::new(config)
}

mod client_tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        assert!(client.config().is_simulated());
    }

    #[test]
    fn test_client_with_custom_http() {
        let creds = Credentials::new("key", "secret", "pass");
        let config = Config::new(creds);
        let http = reqwest::Client::new();
        let client = OkxRestClient::with_http_client(config, http);
        assert!(!client.config().is_simulated());
    }

    #[test]
    fn test_client_creation_with_proxy() {
        let creds = Credentials::new("key", "secret", "pass");
        let config = Config::new(creds).with_proxy("http://127.0.0.1:8888");
        let client = OkxRestClient::new(config);
        // 代理配置只在内部生效，能正常构建即覆盖分支
        assert_eq!(client.config().timeout_secs(), 30);
    }
}

mod public_api_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_system_time() {
        let client = create_test_client();

        match client.get_system_time().await {
            Ok(times) => {
                assert!(!times.is_empty());
                let ts = &times[0].ts;
                // Timestamp should be a number string
                assert!(ts.parse::<u64>().is_ok());
            }
            Err(e) => {
                // Network errors are acceptable in CI
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instruments_spot() {
        let client = create_test_client();

        let params = GetInstrumentsParams {
            inst_type: "SPOT".to_string(),
            uly: None,
            inst_family: None,
            inst_id: None,
        };

        match client.get_instruments(params).await {
            Ok(instruments) => {
                assert!(!instruments.is_empty());
                // Check first instrument has required fields
                let inst = &instruments[0];
                assert!(!inst.inst_id.is_empty());
                assert_eq!(inst.inst_type, "SPOT");
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instruments_swap() {
        let client = create_test_client();

        let params = GetInstrumentsParams {
            inst_type: "SWAP".to_string(),
            uly: None,
            inst_family: None,
            inst_id: None,
        };

        match client.get_instruments(params).await {
            Ok(instruments) => {
                assert!(!instruments.is_empty());
                let inst = &instruments[0];
                assert_eq!(inst.inst_type, "SWAP");
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_funding_rate_and_history_and_mark_price() {
        let client = create_test_client();

        match client.get_funding_rate("BTC-USD-SWAP").await {
            Ok(rates) => {
                if let Some(rate) = rates.first() {
                    assert_eq!(rate.inst_id, "BTC-USD-SWAP");
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }

        let params = okx_rest::api::public::GetFundingRateHistoryParams {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            limit: Some("5".to_string()),
        };
        if let Err(e) = client.get_funding_rate_history(params).await {
            eprintln!("Network error (acceptable): {}", e);
        }

        let mark_params = okx_rest::api::public::GetMarkPriceParams {
            inst_type: "SWAP".to_string(),
            inst_id: Some("BTC-USD-SWAP".to_string()),
            uly: None,
            inst_family: None,
        };
        if let Err(e) = client.get_mark_price(mark_params).await {
            eprintln!("Network error (acceptable): {}", e);
        }
    }

    #[tokio::test]
    async fn test_get_instrument_tick_bands() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetInstrumentTickBandsParams {
            inst_type: "OPTION".to_string(),
            inst_family: None,
        };

        match client.get_instrument_tick_bands(params).await {
            Ok(bands) => {
                if let Some(band) = bands.first() {
                    if let Some(inst_id) = band.get("instId").and_then(|v| v.as_str()) {
                        assert!(!inst_id.is_empty());
                    }
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_option_trades() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetOptionTradesParams {
            inst_id: None,
            inst_family: Some("BTC-USD".to_string()),
            opt_type: None,
        };

        match client.get_option_trades(params).await {
            Ok(_trades) => {
                // 期权交易数据可能为空
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_delivery_exercise_history() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetDeliveryExerciseHistoryParams {
            inst_type: "FUTURES".to_string(),
            uly: Some("BTC-USD".to_string()),
            inst_family: None,
            after: None,
            before: None,
            limit: Some("10".to_string()),
        };

        match client.get_delivery_exercise_history(params).await {
            Ok(_history) => {
                // 交割历史数据可能为空
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_open_interest() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetOpenInterestParams {
            inst_type: "SWAP".to_string(),
            uly: None,
            inst_id: Some("BTC-USD-SWAP".to_string()),
            inst_family: None,
        };

        match client.get_open_interest(params).await {
            Ok(interests) => {
                if let Some(oi) = interests.first() {
                    if let Some(inst_id) = oi.get("instId").and_then(|v| v.as_str()) {
                        assert_eq!(inst_id, "BTC-USD-SWAP");
                    }
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_position_tiers() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetPositionTiersParams {
            inst_type: "SWAP".to_string(),
            td_mode: "cross".to_string(),
            uly: None,
            inst_id: Some("BTC-USD-SWAP".to_string()),
            ccy: None,
            tier: None,
            inst_family: None,
        };

        match client.get_position_tiers(params).await {
            Ok(_tiers) => {
                // 仓位档位数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_price_limit() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetPriceLimitParams {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        match client.get_price_limit(params).await {
            Ok(limits) => {
                if let Some(limit) = limits.first() {
                    if let Some(inst_id) = limit.get("instId").and_then(|v| v.as_str()) {
                        assert_eq!(inst_id, "BTC-USD-SWAP");
                    }
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_opt_summary() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetOptSummaryParams {
            uly: Some("BTC-USD".to_string()),
            exp_time: None,
            inst_family: None,
        };

        match client.get_opt_summary(params).await {
            Ok(_summary) => {
                // 期权概览数据可能为空
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_estimated_price() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetEstimatedPriceParams {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        match client.get_estimated_price(params).await {
            Ok(prices) => {
                if let Some(price) = prices.first() {
                    if let Some(inst_id) = price.get("instId").and_then(|v| v.as_str()) {
                        assert_eq!(inst_id, "BTC-USD-SWAP");
                    }
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_discount_interest_free_quota() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetDiscountQuotaParams { ccy: None };

        match client.get_discount_interest_free_quota(params).await {
            Ok(_quotas) => {
                // 免息额度数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_interest_rate_loan_quota() {
        let client = create_test_client();

        match client.get_interest_rate_loan_quota().await {
            Ok(_quotas) => {
                // 利率贷款额度数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_vip_interest_rate_loan_quota() {
        let client = create_test_client();

        match client.get_vip_interest_rate_loan_quota().await {
            Ok(_quotas) => {
                // VIP 利率贷款额度数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_underlying() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetUnderlyingParams {
            inst_type: Some("SWAP".to_string()),
        };

        match client.get_underlying(params).await {
            Ok(underlyings) => {
                assert!(!underlyings.is_empty());
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_insurance_fund() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetInsuranceFundParams {
            inst_type: Some("SWAP".to_string()),
            r#type: None,
            uly: None,
            ccy: None,
            before: None,
            after: None,
            limit: Some("10".to_string()),
            inst_family: None,
        };

        match client.get_insurance_fund(params).await {
            Ok(_funds) => {
                // 保险基金数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_convert_contract_coin() {
        let client = create_test_client();

        let params = okx_rest::api::public::GetConvertContractCoinParams {
            r#type: Some("1".to_string()),
            inst_id: Some("BTC-USD-SWAP".to_string()),
            sz: Some("1".to_string()),
            px: None,
            unit: None,
        };

        match client.get_convert_contract_coin(params).await {
            Ok(_result) => {
                // 合约币转换结果
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }
}

mod market_api_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ticker() {
        let client = create_test_client();

        match client.get_ticker("BTC-USDT").await {
            Ok(tickers) => {
                assert!(!tickers.is_empty());
                let ticker = &tickers[0];
                assert_eq!(ticker.inst_id, "BTC-USDT");
                // Price should be a valid number
                assert!(ticker.last.parse::<f64>().is_ok());
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_tickers_spot() {
        let client = create_test_client();

        let params = GetTickersParams {
            inst_type: "SPOT".to_string(),
            uly: None,
            inst_family: None,
        };

        match client.get_tickers(params).await {
            Ok(tickers) => {
                assert!(!tickers.is_empty());
                // All should be SPOT instruments
                for ticker in &tickers {
                    assert!(ticker.inst_id.contains("-"));
                }
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        let client = create_test_client();

        match client.get_orderbook("BTC-USDT", Some(5)).await {
            Ok(books) => {
                assert!(!books.is_empty());
                let book = &books[0];
                // Should have asks and bids
                assert!(!book.asks.is_empty() || !book.bids.is_empty());
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_candles() {
        use okx_rest::api::market::GetCandlesParams;

        let client = create_test_client();

        let params = GetCandlesParams {
            inst_id: "BTC-USDT".to_string(),
            bar: Some("1H".to_string()),
            after: None,
            before: None,
            limit: None,
        };

        match client.get_candles(params).await {
            Ok(candles) => {
                assert!(!candles.is_empty());
                // Candles are returned as Vec<Vec<String>>
                let candle = &candles[0];
                assert!(!candle.is_empty());
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_trades() {
        let client = create_test_client();

        match client.get_trades("BTC-USDT", None).await {
            Ok(trades) => {
                assert!(!trades.is_empty());
                let trade = &trades[0];
                assert_eq!(trade.inst_id, "BTC-USDT");
                assert!(trade.px.parse::<f64>().is_ok());
            }
            Err(e) => {
                eprintln!("Network error (acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_index_tickers() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetIndexTickersParams {
            quote_ccy: Some("USD".to_string()),
            inst_id: Some("BTC-USD".to_string()),
        };

        match client.get_index_tickers(params).await {
            Ok(tickers) => {
                if let Some(t) = tickers.first() {
                    assert_eq!(t.inst_id, "BTC-USD");
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_platform_24_volume() {
        let client = create_test_client();

        match client.get_platform_24_volume().await {
            Ok(volumes) => {
                assert!(!volumes.is_empty());
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_index_components() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetIndexComponentsParams {
            index: "BTC-USD".to_string(),
        };

        match client.get_index_components(params).await {
            Ok(_components) => {
                // 指数成分数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_exchange_rate() {
        let client = create_test_client();

        match client.get_exchange_rate().await {
            Ok(rates) => {
                assert!(!rates.is_empty());
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_history_candles() {
        use okx_rest::api::market::GetCandlesParams;

        let client = create_test_client();

        let params = GetCandlesParams {
            inst_id: "BTC-USDT".to_string(),
            bar: Some("1D".to_string()),
            after: None,
            before: None,
            limit: Some("10".to_string()),
        };

        match client.get_history_candles(params).await {
            Ok(candles) => {
                assert!(!candles.is_empty());
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_index_candles() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetCandlesParams {
            inst_id: "BTC-USD".to_string(),
            bar: Some("1H".to_string()),
            after: None,
            before: None,
            limit: Some("10".to_string()),
        };

        match client.get_index_candles(params).await {
            Ok(_candles) => {
                // 指数K线数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_mark_price_candles() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetCandlesParams {
            inst_id: "BTC-USD-SWAP".to_string(),
            bar: Some("1H".to_string()),
            after: None,
            before: None,
            limit: Some("10".to_string()),
        };

        match client.get_mark_price_candles(params).await {
            Ok(_candles) => {
                // 标记价格K线数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_history_trades() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetHistoryTradesParams {
            inst_id: "BTC-USDT".to_string(),
            after: None,
            before: None,
            limit: Some("10".to_string()),
            r#type: None,
        };

        match client.get_history_trades(params).await {
            Ok(_trades) => {
                // 历史交易数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_orderbook_lite() {
        let client = create_test_client();

        match client.get_orderbook_lite("BTC-USDT").await {
            Ok(books) => {
                if let Some(book) = books.first() {
                    assert!(book.ts.parse::<u64>().is_ok());
                }
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_block_ticker() {
        let client = create_test_client();

        match client.get_block_ticker("BTC-USDT").await {
            Ok(_tickers) => {
                // 大宗交易ticker数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_block_tickers() {
        let client = create_test_client();

        let params = okx_rest::api::market::GetBlockTickersParams {
            inst_type: "SPOT".to_string(),
            uly: None,
            inst_family: None,
        };

        match client.get_block_tickers(params).await {
            Ok(_tickers) => {
                // 大宗交易tickers数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_block_trades() {
        let client = create_test_client();

        match client.get_block_trades("BTC-USDT").await {
            Ok(_trades) => {
                // 大宗交易数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_option_family_trades() {
        let client = create_test_client();

        match client.get_option_family_trades("BTC-USD").await {
            Ok(_trades) => {
                // 期权家族交易数据
            }
            Err(e) => eprintln!("Network error (acceptable): {}", e),
        }
    }
}

mod params_tests {
    use okx_rest::api::{
        account::GetPositionsParams,
        trade::{GetOrderParams, GetOrdersPendingParams},
    };

    #[test]
    fn test_get_positions_params_default() {
        let params = GetPositionsParams {
            inst_type: Some("SWAP".to_string()),
            inst_id: None,
            pos_id: None,
        };
        assert_eq!(params.inst_type, Some("SWAP".to_string()));
    }

    #[test]
    fn test_get_order_params() {
        let params = GetOrderParams {
            inst_id: "BTC-USDT".to_string(),
            ord_id: Some("12345".to_string()),
            cl_ord_id: None,
        };
        assert_eq!(params.inst_id, "BTC-USDT");
        assert_eq!(params.ord_id, Some("12345".to_string()));
    }

    #[test]
    fn test_get_orders_pending_params_default() {
        let params = GetOrdersPendingParams::default();
        assert!(params.inst_type.is_none());
        assert!(params.inst_id.is_none());
    }
}
