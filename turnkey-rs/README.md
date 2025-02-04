# Turnkey Rust SDK

Type-safe Rust bindings for the Turnkey API, providing wallet creation and management across multiple blockchain networks.

## Features

- **Multi-Chain Support**

  - Ethereum (Secp256k1)
  - Bitcoin (P2PKH, P2SH, P2WPKH, P2WSH, P2TR)
  - Solana (Ed25519)

- **Type-Safe Wallet Creation**
  - Chain-specific address validation
  - BIP32/44 derivation path standards
  - Configurable wallet parameters

## Structure

```
src/
├── functions/           # High-level API functions
│   ├── create_wallet.rs # Wallet creation implementations
│   └── mod.rs
├── utils/              # Utility functions and helpers
│   ├── wallet.rs       # Wallet-specific utilities
│   └── mod.rs
├── client.rs           # Turnkey client implementation
├── errors.rs           # Error types and handling
└── lib.rs             # Library entry point
```

## Usage

### High-Level API

```rust
use turnkey_rs::{Turnkey, functions::create_wallet::{
    ethereum::EthereumWalletConfig,
    bitcoin::BitcoinP2wpkhWalletConfig,
}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Turnkey::new(
        "your_public_key",
        "your_private_key",
        "your_org_id",
    )?;

    // Create Ethereum wallet with default config
    let eth_config = EthereumWalletConfig::default();
    let eth_wallet = client.create_ethereum_wallet(Some(eth_config)).await?;
    println!("Ethereum wallet created: {}", eth_wallet.address);

    // Create Bitcoin SegWit wallet with custom config
    let btc_config = BitcoinP2wpkhWalletConfig {
        wallet_name: Some("My Bitcoin Wallet".to_string()),
        path_index: Some(0),
        mnemonic_length: Some(24),
    };
    let btc_wallet = client.create_bitcoin_p2wpkh_wallet(Some(btc_config)).await?;
    println!("Bitcoin wallet created: {}", btc_wallet.address);

    Ok(())
}
```

### Raw OpenAPI Usage

```rust
use {
    chrono::Utc,
    std::sync::Arc,
    turnkey_api::{
        apis::wallets_api,
        models::{CreateWalletIntent, CreateWalletRequest, create_wallet_request::Type::ActivityTypeCreateWallet},
    },
    turnkey_rs::{Turnkey, utils},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Turnkey::new(
        "your_public_key",
        "your_private_key",
        "your_org_id",
    )?;

    // Create wallet request
    let create_wallet_body = CreateWalletRequest {
        r#type: ActivityTypeCreateWallet,
        timestamp_ms: Utc::now().timestamp_millis().to_string(),
        organization_id: client.organization_id.clone(),
        parameters: Box::new(CreateWalletIntent {
            wallet_name: "My Wallet".to_string(),
            mnemonic_length: Some(24),
            accounts: vec![utils::wallet::default_ethereum_account()],
        }),
    };

    // Sign request
    let body = serde_json::to_string(&create_wallet_body)?;
    let x_stamp = client.stamp(&body)?;

    // Create request with stamp
    let mut config = (*client.config).clone();
    config.client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-Stamp", x_stamp.parse().unwrap());
            headers
        })
        .build()?;

    // Send request
    let response = wallets_api::create_wallet(&Arc::new(config), create_wallet_body).await?;
    println!("Created wallet: {:?}", response);

    Ok(())
}
```

## License

MIT OR Apache-2.0
