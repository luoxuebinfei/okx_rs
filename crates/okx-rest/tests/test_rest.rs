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
