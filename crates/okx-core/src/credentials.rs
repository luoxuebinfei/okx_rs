//! API credentials management.

use serde::{Deserialize, Serialize};

/// OKX API credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// API key
    api_key: String,
    /// Secret key for signing requests
    secret_key: String,
    /// Passphrase set when creating API key
    passphrase: String,
}

impl Credentials {
    /// Create new credentials.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key from OKX
    /// * `secret_key` - The secret key for signing
    /// * `passphrase` - The passphrase set when creating the API key
    #[must_use]
    pub fn new(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            passphrase: passphrase.into(),
        }
    }

    /// Get the API key.
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the secret key.
    #[must_use]
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }

    /// Get the passphrase.
    #[must_use]
    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }
}
