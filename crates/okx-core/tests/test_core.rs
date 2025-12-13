//! Unit tests for okx-core.

use okx_core::{
    types::{ApiResponse, InstType, OrdType, Side, TdMode},
    Config, Credentials, OkxError, Signer,
};

mod credentials_tests {
    use super::*;

    #[test]
    fn test_credentials_creation() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        assert_eq!(creds.api_key(), "api_key");
        assert_eq!(creds.secret_key(), "secret_key");
        assert_eq!(creds.passphrase(), "passphrase");
    }

    #[test]
    fn test_credentials_with_string_types() {
        let creds = Credentials::new(
            String::from("api_key"),
            String::from("secret_key"),
            String::from("passphrase"),
        );
        assert_eq!(creds.api_key(), "api_key");
    }

    #[test]
    fn test_credentials_clone() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let cloned = creds.clone();
        assert_eq!(cloned.api_key(), creds.api_key());
    }
}

mod config_tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds);

        assert!(!config.is_simulated());
        assert_eq!(config.timeout_secs(), 30);
        assert!(config.rest_url().contains("okx.com"));
        assert!(config.proxy_url().is_none());
    }

    #[test]
    fn test_config_simulated() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds).simulated(true);

        assert!(config.is_simulated());
        assert!(config.ws_public_url().contains("wspap"));
        assert!(config.ws_private_url().contains("wspap"));
    }

    #[test]
    fn test_config_simulated_toggle_back_to_live() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds).simulated(true).simulated(false);

        assert!(!config.is_simulated());
        assert!(config.ws_public_url().contains("ws.okx.com"));
        assert!(config.ws_private_url().contains("ws.okx.com"));
    }

    #[test]
    fn test_config_custom_timeout() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds).with_timeout_secs(60);

        assert_eq!(config.timeout_secs(), 60);
    }

    #[test]
    fn test_config_with_proxy() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds).with_proxy("http://127.0.0.1:7890");

        assert_eq!(config.proxy_url(), Some("http://127.0.0.1:7890"));
    }

    #[test]
    fn test_config_custom_urls() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let config = Config::new(creds)
            .with_rest_url("https://custom.api.com")
            .with_ws_public_url("wss://custom.ws.com/public")
            .with_ws_private_url("wss://custom.ws.com/private");

        assert_eq!(config.rest_url(), "https://custom.api.com");
        assert_eq!(config.ws_public_url(), "wss://custom.ws.com/public");
        assert_eq!(config.ws_private_url(), "wss://custom.ws.com/private");
    }
}

mod signer_tests {
    use super::*;

    #[test]
    fn test_timestamp_format() {
        let ts = Signer::timestamp();
        // Format: 2024-01-01T12:00:00.000Z
        assert!(ts.contains("T"));
        assert!(ts.ends_with("Z"));
        assert!(ts.len() >= 20);
    }

    #[test]
    fn test_pre_hash_get() {
        let pre_hash = Signer::pre_hash(
            "2024-01-01T12:00:00.000Z",
            "GET",
            "/api/v5/account/balance",
            "",
        );
        assert_eq!(
            pre_hash,
            "2024-01-01T12:00:00.000ZGET/api/v5/account/balance"
        );
    }

    #[test]
    fn test_pre_hash_post() {
        let body = r#"{"instId":"BTC-USDT"}"#;
        let pre_hash = Signer::pre_hash(
            "2024-01-01T12:00:00.000Z",
            "POST",
            "/api/v5/trade/order",
            body,
        );
        assert_eq!(
            pre_hash,
            r#"2024-01-01T12:00:00.000ZPOST/api/v5/trade/order{"instId":"BTC-USDT"}"#
        );
    }

    #[test]
    fn test_pre_hash_method_uppercase() {
        let pre_hash = Signer::pre_hash("ts", "get", "/path", "");
        assert!(pre_hash.contains("GET"));
    }

    #[test]
    fn test_sign_produces_base64() {
        let signature = Signer::sign("test_message", "test_secret");
        // Base64 characters only
        assert!(signature
            .chars()
            .all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '='));
    }

    #[test]
    fn test_sign_deterministic() {
        let sig1 = Signer::sign("message", "secret");
        let sig2 = Signer::sign("message", "secret");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_sign_different_messages() {
        let sig1 = Signer::sign("message1", "secret");
        let sig2 = Signer::sign("message2", "secret");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_generate_headers() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let signer = Signer::new(creds);

        let headers = signer.generate_headers("GET", "/api/v5/account/balance", "", false);

        assert!(headers.iter().any(|(k, _)| *k == "OK-ACCESS-KEY"));
        assert!(headers.iter().any(|(k, _)| *k == "OK-ACCESS-SIGN"));
        assert!(headers.iter().any(|(k, _)| *k == "OK-ACCESS-TIMESTAMP"));
        assert!(headers.iter().any(|(k, _)| *k == "OK-ACCESS-PASSPHRASE"));
        // 按对齐策略：仅在 simulated=true 时发送 x-simulated-trading: 1
        assert!(!headers.iter().any(|(k, _)| *k == "x-simulated-trading"));
    }

    #[test]
    fn test_generate_headers_simulated() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let signer = Signer::new(creds);

        let headers = signer.generate_headers("GET", "/path", "", true);
        let sim_header = headers
            .iter()
            .find(|(k, _)| *k == "x-simulated-trading")
            .unwrap();
        assert_eq!(sim_header.1, "1");
    }

    #[test]
    fn test_generate_public_headers() {
        let headers = Signer::generate_public_headers(false);
        assert!(headers.iter().any(|(k, _)| *k == "Content-Type"));
        // 按对齐策略：仅在 simulated=true 时发送 x-simulated-trading: 1
        assert!(!headers.iter().any(|(k, _)| *k == "x-simulated-trading"));
        // Should NOT have auth headers
        assert!(!headers.iter().any(|(k, _)| *k == "OK-ACCESS-KEY"));
    }

    #[test]
    fn test_generate_public_headers_simulated() {
        let headers = Signer::generate_public_headers(true);
        let sim_header = headers
            .iter()
            .find(|(k, _)| *k == "x-simulated-trading")
            .expect("simulated=true 时必须包含 x-simulated-trading");
        assert_eq!(sim_header.1, "1");
    }

    #[test]
    fn test_ws_login_params() {
        let creds = Credentials::new("api_key", "secret_key", "passphrase");
        let signer = Signer::new(creds);

        let (api_key, passphrase, timestamp, sign) = signer.generate_ws_login_params();

        assert_eq!(api_key, "api_key");
        assert_eq!(passphrase, "passphrase");
        assert!(!timestamp.is_empty());
        assert!(!sign.is_empty());
    }
}

mod error_tests {
    use super::*;

    #[test]
    fn test_api_error_creation() {
        let err = OkxError::api("50001", "Invalid parameter");
        match err {
            OkxError::Api { code, msg } => {
                assert_eq!(code, "50001");
                assert_eq!(msg, "Invalid parameter");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[test]
    fn test_is_api_error() {
        let err = OkxError::api("50001", "Invalid parameter");
        assert!(err.is_api_error("50001"));
        assert!(!err.is_api_error("50002"));
    }

    #[test]
    fn test_error_display() {
        let err = OkxError::Http("connection refused".to_string());
        assert!(err.to_string().contains("connection refused"));

        let err = OkxError::api("50001", "Invalid parameter");
        assert!(err.to_string().contains("50001"));
        assert!(err.to_string().contains("Invalid parameter"));
    }

    #[test]
    fn test_serde_error_conversion() {
        let json_err = serde_json::from_str::<i32>("invalid").unwrap_err();
        let okx_err: OkxError = json_err.into();
        assert!(matches!(okx_err, OkxError::Serde(_)));
    }
}

mod types_tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response: ApiResponse<String> = ApiResponse {
            code: "0".to_string(),
            msg: "".to_string(),
            data: vec!["test".to_string()],
        };
        assert!(response.is_success());
    }

    #[test]
    fn test_api_response_failure() {
        let response: ApiResponse<String> = ApiResponse {
            code: "50001".to_string(),
            msg: "Invalid parameter".to_string(),
            data: vec![],
        };
        assert!(!response.is_success());
    }

    #[test]
    fn test_inst_type_as_str() {
        assert_eq!(InstType::Spot.as_str(), "SPOT");
        assert_eq!(InstType::Margin.as_str(), "MARGIN");
        assert_eq!(InstType::Swap.as_str(), "SWAP");
        assert_eq!(InstType::Futures.as_str(), "FUTURES");
        assert_eq!(InstType::Option.as_str(), "OPTION");
    }

    #[test]
    fn test_td_mode_as_str() {
        assert_eq!(TdMode::Cash.as_str(), "cash");
        assert_eq!(TdMode::Cross.as_str(), "cross");
        assert_eq!(TdMode::Isolated.as_str(), "isolated");
    }

    #[test]
    fn test_side_as_str() {
        assert_eq!(Side::Buy.as_str(), "buy");
        assert_eq!(Side::Sell.as_str(), "sell");
    }

    #[test]
    fn test_ord_type_as_str() {
        assert_eq!(OrdType::Market.as_str(), "market");
        assert_eq!(OrdType::Limit.as_str(), "limit");
        assert_eq!(OrdType::PostOnly.as_str(), "post_only");
        assert_eq!(OrdType::Fok.as_str(), "fok");
        assert_eq!(OrdType::Ioc.as_str(), "ioc");
    }

    #[test]
    fn test_inst_type_serde() {
        let json = serde_json::to_string(&InstType::Spot).unwrap();
        assert_eq!(json, r#""SPOT""#);

        let parsed: InstType = serde_json::from_str(r#""SWAP""#).unwrap();
        assert_eq!(parsed, InstType::Swap);
    }

    #[test]
    fn test_side_serde() {
        let json = serde_json::to_string(&Side::Buy).unwrap();
        assert_eq!(json, r#""buy""#);

        let parsed: Side = serde_json::from_str(r#""sell""#).unwrap();
        assert_eq!(parsed, Side::Sell);
    }
}
