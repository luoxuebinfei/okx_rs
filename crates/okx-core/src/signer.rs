//! Request signing for OKX API authentication.
//!
//! ## Authentication Process (verified from official Python SDK)
//!
//! Source: `okx/utils.py` in <https://github.com/okxapi/python-okx>
//!
//! 1. Create pre-hash string: `timestamp + METHOD + requestPath + body`
//! 2. Sign with HMAC-SHA256 using `secret_key`
//! 3. Base64 encode the signature
//!
//! ## Header Names
//!
//! - `OK-ACCESS-KEY`: API key
//! - `OK-ACCESS-SIGN`: Signature
//! - `OK-ACCESS-TIMESTAMP`: ISO 8601 timestamp
//! - `OK-ACCESS-PASSPHRASE`: Passphrase
//! - `x-simulated-trading`: "0" for live, "1" for demo

use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::Credentials;

type HmacSha256 = Hmac<Sha256>;

/// HTTP header names for OKX API authentication.
pub mod headers {
    /// Content-Type header
    pub const CONTENT_TYPE: &str = "Content-Type";
    /// API key header
    pub const OK_ACCESS_KEY: &str = "OK-ACCESS-KEY";
    /// Signature header
    pub const OK_ACCESS_SIGN: &str = "OK-ACCESS-SIGN";
    /// Timestamp header
    pub const OK_ACCESS_TIMESTAMP: &str = "OK-ACCESS-TIMESTAMP";
    /// Passphrase header
    pub const OK_ACCESS_PASSPHRASE: &str = "OK-ACCESS-PASSPHRASE";
    /// Simulated trading flag header
    pub const X_SIMULATED_TRADING: &str = "x-simulated-trading";
    /// Content-Type value for JSON
    pub const APPLICATION_JSON: &str = "application/json";
}

/// Request signer for OKX API authentication.
#[derive(Debug, Clone)]
pub struct Signer {
    credentials: Credentials,
}

impl Signer {
    /// Create a new signer with the given credentials.
    #[must_use]
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }

    /// Get the current timestamp in ISO 8601 format.
    ///
    /// Format: `2024-01-01T12:00:00.000Z`
    #[must_use]
    pub fn timestamp() -> String {
        Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    }

    /// Create the pre-hash string for signing.
    ///
    /// Format: `timestamp + METHOD + requestPath + body`
    ///
    /// ## Arguments
    ///
    /// * `timestamp` - ISO 8601 timestamp
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `request_path` - Request path including query string (e.g., `/api/v5/account/balance?ccy=BTC`)
    /// * `body` - Request body (empty string for GET requests)
    #[must_use]
    pub fn pre_hash(timestamp: &str, method: &str, request_path: &str, body: &str) -> String {
        format!(
            "{}{}{}{}",
            timestamp,
            method.to_uppercase(),
            request_path,
            body
        )
    }

    /// Sign a message with HMAC-SHA256 and Base64 encode.
    ///
    /// ## Arguments
    ///
    /// * `message` - The message to sign (pre-hash string)
    /// * `secret_key` - The secret key for signing
    ///
    /// ## Panics
    ///
    /// 当 HMAC 初始化失败时会触发 panic。按 HMAC 规范密钥长度不受限，因此该情况理论上不会发生；
    /// 若发生，通常意味着底层依赖或构建环境异常。
    #[must_use]
    pub fn sign(message: &str, secret_key: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        STANDARD.encode(result.into_bytes())
    }

    /// Generate authentication headers for a request.
    ///
    /// ## Arguments
    ///
    /// * `method` - HTTP method
    /// * `request_path` - Request path including query string
    /// * `body` - Request body
    /// * `simulated` - Whether to use simulated trading
    ///
    /// ## Returns
    ///
    /// A vector of (`header_name`, `header_value`) pairs.
    #[must_use]
    pub fn generate_headers(
        &self,
        method: &str,
        request_path: &str,
        body: &str,
        simulated: bool,
    ) -> Vec<(&'static str, String)> {
        let timestamp = Self::timestamp();
        let pre_hash = Self::pre_hash(&timestamp, method, request_path, body);
        let signature = Self::sign(&pre_hash, self.credentials.secret_key());

        let mut headers = vec![
            (headers::CONTENT_TYPE, headers::APPLICATION_JSON.to_string()),
            (
                headers::OK_ACCESS_KEY,
                self.credentials.api_key().to_string(),
            ),
            (headers::OK_ACCESS_SIGN, signature),
            (headers::OK_ACCESS_TIMESTAMP, timestamp),
            (
                headers::OK_ACCESS_PASSPHRASE,
                self.credentials.passphrase().to_string(),
            ),
        ];

        if simulated {
            headers.push((headers::X_SIMULATED_TRADING, "1".to_string()));
        }

        headers
    }

    /// Generate headers for requests that don't require authentication.
    ///
    /// ## Arguments
    ///
    /// * `simulated` - Whether to use simulated trading
    #[must_use]
    pub fn generate_public_headers(simulated: bool) -> Vec<(&'static str, String)> {
        let mut headers = vec![(headers::CONTENT_TYPE, headers::APPLICATION_JSON.to_string())];

        if simulated {
            headers.push((headers::X_SIMULATED_TRADING, "1".to_string()));
        }

        headers
    }

    /// Generate login parameters for WebSocket authentication.
    ///
    /// For WebSocket login, the message is: `timestamp + 'GET' + '/users/self/verify'`
    ///
    /// ## Returns
    ///
    /// A tuple of (`api_key`, `passphrase`, `timestamp`, `sign`)
    #[must_use]
    pub fn generate_ws_login_params(&self) -> (String, String, String, String) {
        let timestamp = Utc::now().timestamp().to_string();
        let message = format!("{timestamp}GET/users/self/verify");
        let signature = Self::sign(&message, self.credentials.secret_key());

        (
            self.credentials.api_key().to_string(),
            self.credentials.passphrase().to_string(),
            timestamp,
            signature,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pre_hash() {
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
    fn test_pre_hash_with_body() {
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
    fn test_sign() {
        // Test that sign produces a base64 encoded string
        let signature = Signer::sign("test_message", "test_secret");
        // Verify it's valid base64
        assert!(STANDARD.decode(&signature).is_ok());
    }

    #[test]
    fn generate_public_headers_sets_simulated_flag() {
        let headers = Signer::generate_public_headers(true);
        let map: HashMap<_, _> = headers.into_iter().collect();
        assert_eq!(map[headers::CONTENT_TYPE], headers::APPLICATION_JSON);
        assert_eq!(map[headers::X_SIMULATED_TRADING], "1");

        let headers_live = Signer::generate_public_headers(false);
        let map_live: HashMap<_, _> = headers_live.into_iter().collect();
        assert!(!map_live.contains_key(headers::X_SIMULATED_TRADING));
    }

    #[test]
    fn generate_headers_builds_expected_signature_and_fields() {
        let signer = Signer::new(Credentials::new("api", "secret", "pass"));
        let body = r#"{"k":1}"#;
        let headers = signer.generate_headers("post", "/api/v5/unit", body, true);
        let map: HashMap<_, _> = headers.into_iter().collect();

        assert_eq!(map[headers::OK_ACCESS_KEY], "api");
        assert_eq!(map[headers::OK_ACCESS_PASSPHRASE], "pass");
        assert_eq!(map[headers::X_SIMULATED_TRADING], "1");
        assert_eq!(map[headers::CONTENT_TYPE], headers::APPLICATION_JSON);

        let ts = &map[headers::OK_ACCESS_TIMESTAMP];
        let pre_hash = Signer::pre_hash(ts, "POST", "/api/v5/unit", body);
        let expected_sign = Signer::sign(&pre_hash, "secret");
        assert_eq!(map[headers::OK_ACCESS_SIGN], expected_sign);

        let headers_live = signer.generate_headers("post", "/api/v5/unit", body, false);
        let map_live: HashMap<_, _> = headers_live.into_iter().collect();
        assert!(!map_live.contains_key(headers::X_SIMULATED_TRADING));
    }

    #[test]
    fn ws_login_params_signature_matches_timestamp() {
        let signer = Signer::new(Credentials::new("api", "secret", "pass"));
        let (api_key, passphrase, ts, sign) = signer.generate_ws_login_params();

        assert_eq!(api_key, "api");
        assert_eq!(passphrase, "pass");
        let expected = Signer::sign(&format!("{ts}GET/users/self/verify"), "secret");
        assert_eq!(sign, expected);
    }
}
