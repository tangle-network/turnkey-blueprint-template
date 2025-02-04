use {
    crate::{
        client::Turnkey,
        errors::{TurnkeyError, TurnkeyResult},
        utils::wallet,
    },
    chrono::Utc,
    rand::{Rng, distr::Alphanumeric},
    std::sync::Arc,
    turnkey_api::{
        apis::wallets_api,
        models::{
            CreateWalletIntent, CreateWalletRequest,
            create_wallet_request::Type::ActivityTypeCreateWallet,
        },
    },
};

/// Macro to implement wallet creation functionality for different chains
macro_rules! impl_wallet_creation {
    (
        $wallet_type:ident,
        $account_fn:path,
        $address_type:ty,
        $address_validation:expr
    ) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<$wallet_type WalletConfig>] {
                /// Custom name for the wallet. If None, a random name will be generated
                pub wallet_name: Option<String>,
                /// Custom path index for derivation. If None, defaults to 0
                pub path_index: Option<u32>,
                /// Length of mnemonic phrase. If None, defaults to 24 words
                pub mnemonic_length: Option<u32>,
            }

            impl Default for [<$wallet_type WalletConfig>] {
                fn default() -> Self {
                    Self {
                        wallet_name: None,
                        path_index: None,
                        mnemonic_length: Some(24),
                    }
                }
            }

            #[derive(Debug, Clone)]
            pub struct [<$wallet_type WalletResponse>] {
                /// The unique identifier for the created wallet
                pub wallet_id: String,
                /// The wallet's name
                pub wallet_name: String,
                /// The wallet's address
                pub address: $address_type,
            }

            impl Turnkey {
                paste::paste! {
                    pub async fn [<create_ $wallet_type:snake _wallet>](
                        &self,
                        config: Option<[<$wallet_type WalletConfig>]>,
                    ) -> TurnkeyResult<[<$wallet_type WalletResponse>]> {
                        let random_suffix: String = rand::rng()
                            .sample_iter(&Alphanumeric)
                            .take(4)
                            .map(char::from)
                            .collect();

                        let config = config.unwrap_or_default();
                        let wallet_name = config
                            .wallet_name
                            .unwrap_or_else(|| format!(concat!(stringify!($wallet_type), " Wallet {}"), random_suffix));
                        let path_index = config.path_index.unwrap_or(0);

                        let parameters = CreateWalletIntent {
                            wallet_name: wallet_name.clone(),
                            mnemonic_length: config.mnemonic_length.map(|x| x as i32),
                            accounts: vec![$account_fn(path_index)],
                        };

                        let timestamp_ms = Utc::now().timestamp_millis().to_string();
                        let create_wallet_body = CreateWalletRequest {
                            r#type: ActivityTypeCreateWallet,
                            timestamp_ms: timestamp_ms.clone(),
                            organization_id: self.organization_id.clone(),
                            parameters: Box::new(parameters.clone()),
                        };

                        let body = serde_json::to_string(&create_wallet_body)?;
                        let x_stamp = self.stamp(&body)?;

                        let mut config = (*self.config).clone();
                        config.client = reqwest::Client::builder()
                            .default_headers({
                                let mut headers = reqwest::header::HeaderMap::new();
                                headers.insert("X-Stamp", x_stamp.parse().unwrap());
                                headers
                            })
                            .build()
                            .map_err(|e| TurnkeyError::OtherError(format!("Failed to build client: {}", e)))?;
                        let config = Arc::new(config);
                        let response = wallets_api::create_wallet(&config, CreateWalletRequest {
                                organization_id: self.organization_id.clone(),
                                parameters: Box::new(parameters),
                                timestamp_ms,
                                r#type: ActivityTypeCreateWallet,
                            })
                            .await
                            .map_err(|e| TurnkeyError::OtherError(format!("Failed to create wallet: {}", e)))?;

                        let create_wallet_result = response
                            .activity
                            .result
                            .create_wallet_result
                            .ok_or_else(|| TurnkeyError::OtherError("Missing intent in response".into()))?;

                        let wallet_id = create_wallet_result.wallet_id;
                        let address_str = &create_wallet_result.addresses[0];
                        let validation_fn = $address_validation;
                        let address = validation_fn(address_str)
                            .map_err(|e| TurnkeyError::OtherError(format!("Invalid address: {}", e)))?;

                        Ok([<$wallet_type WalletResponse>] {
                            wallet_id,
                            wallet_name,
                            address,
                        })
                    }
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[tokio::test]
                async fn [<test_create_ $wallet_type:snake _wallet>]() -> TurnkeyResult<()> {
                    let client = Turnkey::new()?;

                    let wallet = client.[<create_ $wallet_type:snake _wallet>](None).await?;
                    assert!(wallet.wallet_name.starts_with(concat!(stringify!($wallet_type), " Wallet")));
                    assert!(!wallet.wallet_id.is_empty());

                    let config = [<$wallet_type WalletConfig>] {
                        wallet_name: Some("Test Wallet".to_string()),
                        path_index: Some(0),
                        mnemonic_length: Some(24),
                    };
                    let custom_wallet = client.[<create_ $wallet_type:snake _wallet>](Some(config)).await?;
                    assert_eq!(custom_wallet.wallet_name, "Test Wallet");
                    assert!(!custom_wallet.wallet_id.is_empty());

                    Ok(())
                }
            }
        }
    };
}

pub mod ethereum {
    use std::str::FromStr;

    use super::*;
    use alloy_primitives::Address as EthereumAddress;

    fn validate_eth_address(addr: &str) -> Result<EthereumAddress, String> {
        EthereumAddress::from_str(addr).map_err(|e| format!("Invalid Ethereum address: {}", e))
    }

    impl_wallet_creation!(
        Ethereum,
        wallet::ethereum_account,
        EthereumAddress,
        validate_eth_address
    );
}

pub mod solana {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    fn validate_solana_address(addr: &str) -> Result<Pubkey, solana_sdk::pubkey::ParsePubkeyError> {
        Pubkey::try_from(addr)
    }

    impl_wallet_creation!(
        Solana,
        wallet::solana_account,
        Pubkey,
        validate_solana_address
    );
}

pub mod bitcoin {
    use super::*;
    use ::bitcoin::{
        Address, Network,
        address::{Payload, WitnessVersion},
    };
    use std::str::FromStr;

    fn validate_p2pkh_address(addr: &str) -> Result<Address, ::bitcoin::address::Error> {
        let address = Address::from_str(addr)?;
        let address = address.require_network(Network::Bitcoin)?;

        match address.payload {
            Payload::PubkeyHash(_) => Ok(address),
            _ => Err(::bitcoin::address::Error::UnrecognizedScript),
        }
    }

    fn validate_p2sh_address(addr: &str) -> Result<Address, ::bitcoin::address::Error> {
        let address = Address::from_str(addr)?;
        let address = address.require_network(Network::Bitcoin)?;

        match address.payload {
            Payload::ScriptHash(_) => Ok(address),
            _ => Err(::bitcoin::address::Error::UnrecognizedScript),
        }
    }

    fn validate_p2wpkh_address(addr: &str) -> Result<Address, ::bitcoin::address::Error> {
        let address = Address::from_str(addr)?;
        let address = address.require_network(Network::Bitcoin)?;

        match address.clone().payload {
            Payload::WitnessProgram(wp)
                if wp.version() == WitnessVersion::V0 && wp.program().len() == 20 =>
            {
                Ok(address)
            }
            _ => Err(::bitcoin::address::Error::UnrecognizedScript),
        }
    }

    fn validate_p2wsh_address(addr: &str) -> Result<Address, ::bitcoin::address::Error> {
        let address = Address::from_str(addr)?;
        let address = address.require_network(Network::Bitcoin)?;

        match address.clone().payload {
            Payload::WitnessProgram(wp)
                if wp.version() == WitnessVersion::V0 && wp.program().len() == 32 =>
            {
                Ok(address)
            }
            _ => Err(::bitcoin::address::Error::UnrecognizedScript),
        }
    }

    fn validate_p2tr_address(addr: &str) -> Result<Address, ::bitcoin::address::Error> {
        let address = Address::from_str(addr)?;
        let address = address.require_network(Network::Bitcoin)?;

        match address.clone().payload {
            Payload::WitnessProgram(wp) if wp.version() == WitnessVersion::V1 => Ok(address),
            _ => Err(::bitcoin::address::Error::UnrecognizedScript),
        }
    }

    // Implement different Bitcoin wallet types
    impl_wallet_creation!(
        BitcoinP2pkh,
        wallet::bitcoin_mainnet::p2pkh_account,
        Address,
        validate_p2pkh_address
    );

    impl_wallet_creation!(
        BitcoinP2sh,
        wallet::bitcoin_mainnet::p2sh_account,
        Address,
        validate_p2sh_address
    );

    impl_wallet_creation!(
        BitcoinP2wpkh,
        wallet::bitcoin_mainnet::p2wpkh_account,
        Address,
        validate_p2wpkh_address
    );

    impl_wallet_creation!(
        BitcoinP2wsh,
        wallet::bitcoin_mainnet::p2wsh_account,
        Address,
        validate_p2wsh_address
    );

    impl_wallet_creation!(
        BitcoinP2tr,
        wallet::bitcoin_mainnet::p2tr_account,
        Address,
        validate_p2tr_address
    );
}
