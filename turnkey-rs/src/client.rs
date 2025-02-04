use {
    crate::{
        ApiStamp,
        bytes::{bytes_to_hex, hex_to_bytes},
        errors::TurnkeyResult,
    },
    base64_url,
    p256::ecdsa::{SigningKey, signature::Signer},
    std::sync::Arc,
    turnkey_api::apis::configuration::{ApiKey, Configuration},
};

const API_BASE_PATH: &str = "https://api.turnkey.com";

/// Represents the Turnkey service client, encapsulating API configuration and signing capabilities.
pub struct Turnkey {
    pub api_public_key: String,
    api_private_key: String,
    pub organization_id: String,
    signing_key: SigningKey,
    pub config: Arc<Configuration>,
}

impl Turnkey {
    /// Creates a new instance of the Turnkey client.
    ///
    /// # Arguments
    ///
    /// * `api_public_key` - Your API public key from Turnkey dashboard
    /// * `api_private_key` - Your API private key from Turnkey dashboard
    /// * `organization_id` - Your organization ID from Turnkey dashboard
    ///
    /// # Example
    ///
    /// ```rust
    /// use turnkey_rs::Turnkey;
    ///
    /// let client = Turnkey::new(
    ///     "your_public_key",
    ///     "your_private_key",
    ///     "your_org_id"
    /// );
    /// ```
    pub fn new(
        api_public_key: impl Into<String>,
        api_private_key: impl Into<String>,
        organization_id: impl Into<String>,
    ) -> TurnkeyResult<Self> {
        let api_public_key = api_public_key.into();
        let api_private_key = api_private_key.into();
        let organization_id = organization_id.into();

        // Create signing key
        let private_key_bytes = hex_to_bytes(&api_private_key)?;
        let signing_key = SigningKey::from_slice(&private_key_bytes.as_slice())?;

        // Create base configuration
        let config = Configuration {
            base_path: API_BASE_PATH.to_string(),
            user_agent: Some("turnkey-rs".to_string()),
            api_key: Some(ApiKey {
                prefix: Some("Bearer".to_string()),
                key: api_public_key.clone(),
            }),
            bearer_access_token: None,
            basic_auth: None,
            oauth_access_token: None,
            client: reqwest::Client::new(),
        };

        Ok(Self {
            api_public_key,
            api_private_key,
            organization_id,
            signing_key,
            config: Arc::new(config),
        })
    }

    /// Creates a digital stamp for API authentication
    pub fn stamp(&self, message: &str) -> TurnkeyResult<String> {
        let signature: p256::ecdsa::Signature = self.signing_key.sign(message.as_bytes());
        let signature_der = signature.to_der().to_bytes();
        let signature_hex = bytes_to_hex(&signature_der)?;

        let stamp = ApiStamp {
            public_key: self.api_public_key.clone(),
            signature: signature_hex,
            scheme: "TK_API_P256",
        };

        let stamp_json = serde_json::to_string(&stamp)?;
        Ok(base64_url::encode(&stamp_json))
    }
}
