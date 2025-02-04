use turnkey_api::models::{AddressFormat, Curve, PathFormat, WalletAccountParams};

/// Creates a wallet account with the specified curve, derivation path, and address format.
///
/// # Arguments
///
/// * `curve` - The elliptic curve to use for key generation (e.g. Secp256k1, Ed25519)
/// * `path` - The BIP32 derivation path as a string
/// * `address_format` - The format to use when generating addresses
///
/// # Returns
///
/// A `WalletAccountParams` struct configured with the provided parameters and BIP32 path format
fn create_wallet_account(
    curve: Curve,
    path: String,
    address_format: AddressFormat,
) -> WalletAccountParams {
    WalletAccountParams {
        curve,
        path_format: PathFormat::PathFormatBip32,
        path,
        address_format,
    }
}

/// Macro to create account functions for a specific blockchain chain.
///
/// This macro generates a function that creates wallet account parameters for a specific blockchain,
/// following BIP44 derivation path standards with customization options.
///
/// # Arguments
///
/// * `$name` - The name of the function to generate
/// * `$curve` - The elliptic curve to use (e.g. `Curve::CurveSecp256K1`)
/// * `$coin_type` - The BIP44 coin type number (e.g. "0" for Bitcoin)
/// * `$format` - The address format to use
/// * `$extra_path` - Optional additional path components to append
///
/// # Example
///
/// ```rust
/// define_chain_account!(
///     /// Creates an Ethereum account
///     ethereum_account,
///     Curve::CurveSecp256K1,
///     "60",
///     AddressFormat::AddressFormatEthereum,
///     "/0/0"
/// );
/// ```
macro_rules! define_chain_account {
    (
        $(#[$meta:meta])*
        $name:ident,
        $curve:expr,
        $coin_type:expr,
        $format:expr
        $(, $extra_path:expr)?
    ) => {
        $(#[$meta])*
        pub fn $name(path_index: u32) -> WalletAccountParams {
            create_wallet_account(
                $curve,
                format!(concat!("m/44'/", $coin_type, "/{}'", $($extra_path)?), path_index),
                $format,
            )
        }
    };
}

/// Macro to create Bitcoin-style account functions for a specific network (mainnet or testnet).
///
/// This macro generates a module containing functions for creating different types of Bitcoin
/// wallet accounts, including:
/// - P2PKH (Pay to Public Key Hash)
/// - P2WPKH (Pay to Witness Public Key Hash / Native SegWit)
/// - P2WSH (Pay to Witness Script Hash)
/// - P2TR (Pay to Taproot)
/// - P2SH (Pay to Script Hash)
///
/// Each function follows standard BIP derivation paths for that address type.
///
/// # Arguments
///
/// * `$network` - The network identifier (e.g. `mainnet`, `testnet`)
/// * `$format_prefix` - The address format prefix to use in the generated code
///
/// # Example
///
/// ```rust
/// define_bitcoin_accounts!(mainnet, "");  // For mainnet addresses
/// define_bitcoin_accounts!(testnet, "Testnet"); // For testnet addresses
/// ```
macro_rules! define_bitcoin_accounts {
    ($network:ident, $format_prefix:expr) => {
        paste::paste! {
            pub mod [<bitcoin_ $network>] {
                use super::*;

                pub fn p2pkh_account(path_index: u32) -> WalletAccountParams {
                    create_wallet_account(
                        Curve::CurveSecp256K1,
                        format!("m/44'/0'/{path_index}'/0/0"),
                        AddressFormat::[<AddressFormatBitcoin $format_prefix P2Pkh>],
                    )
                }

                pub fn p2wpkh_account(path_index: u32) -> WalletAccountParams {
                    create_wallet_account(
                        Curve::CurveSecp256K1,
                        format!("m/84'/0'/{path_index}'/0/0"),
                        AddressFormat::[<AddressFormatBitcoin $format_prefix P2Wpkh>],
                    )
                }

                pub fn p2wsh_account(path_index: u32) -> WalletAccountParams {
                    create_wallet_account(
                        Curve::CurveSecp256K1,
                        format!("m/48'/0'/{path_index}'/2'/0/0"),
                        AddressFormat::[<AddressFormatBitcoin $format_prefix P2Wsh>],
                    )
                }

                pub fn p2tr_account(path_index: u32) -> WalletAccountParams {
                    create_wallet_account(
                        Curve::CurveSecp256K1,
                        format!("m/86'/0'/{path_index}'/0/0"),
                        AddressFormat::[<AddressFormatBitcoin $format_prefix P2Tr>],
                    )
                }

                pub fn p2sh_account(path_index: u32) -> WalletAccountParams {
                    create_wallet_account(
                        Curve::CurveSecp256K1,
                        format!("m/44'/0'/{path_index}'/0/0"),
                        AddressFormat::[<AddressFormatBitcoin $format_prefix P2Sh>],
                    )
                }
            }
        }
    };
}

// SECP256K1 Chain Accounts
define_chain_account!(
    ethereum_account,
    Curve::CurveSecp256K1,
    "60",
    AddressFormat::AddressFormatEthereum,
    "/0/0"
);

define_chain_account!(
    cosmos_account,
    Curve::CurveSecp256K1,
    "118",
    AddressFormat::AddressFormatCosmos,
    "/0/0"
);

define_chain_account!(
    tron_account,
    Curve::CurveSecp256K1,
    "195",
    AddressFormat::AddressFormatTron
);

// Bitcoin network implementations
define_bitcoin_accounts!(mainnet, "Mainnet");
define_bitcoin_accounts!(testnet, "Testnet");
define_bitcoin_accounts!(signet, "Signet");
define_bitcoin_accounts!(regtest, "Regtest");

// Dogecoin accounts
pub mod dogecoin {
    use super::*;

    define_chain_account!(
        mainnet_account,
        Curve::CurveSecp256K1,
        "3",
        AddressFormat::AddressFormatDogeMainnet,
        "/0/0"
    );

    define_chain_account!(
        testnet_account,
        Curve::CurveSecp256K1,
        "3",
        AddressFormat::AddressFormatDogeTestnet,
        "/0/0"
    );

    pub fn default_mainnet_account() -> WalletAccountParams {
        mainnet_account(0)
    }

    pub fn default_testnet_account() -> WalletAccountParams {
        testnet_account(0)
    }
}

// ED25519 Chain Accounts
define_chain_account!(
    solana_account,
    Curve::CurveEd25519,
    "501",
    AddressFormat::AddressFormatSolana,
    "/0'"
);

define_chain_account!(
    sui_account,
    Curve::CurveEd25519,
    "784",
    AddressFormat::AddressFormatSui,
    "/0'/0'"
);

define_chain_account!(
    /// Creates an Aptos account at the specified index
    aptos_account,
    Curve::CurveEd25519,
    "637",
    AddressFormat::AddressFormatAptos,
    "/0'/0'"
);

define_chain_account!(
    xlm_account,
    Curve::CurveEd25519,
    "148",
    AddressFormat::AddressFormatXlm
);

// TON accounts
pub mod ton {
    use super::*;

    define_chain_account!(
        v3r2_account,
        Curve::CurveEd25519,
        "607",
        AddressFormat::AddressFormatTonV3R2,
        "/0'/0'"
    );

    define_chain_account!(
        v4r2_account,
        Curve::CurveEd25519,
        "607",
        AddressFormat::AddressFormatTonV4R2,
        "/0'/0'"
    );
}

// Default accounts for each blockchain
pub fn default_ethereum_account() -> WalletAccountParams {
    ethereum_account(0)
}

pub fn default_cosmos_account() -> WalletAccountParams {
    cosmos_account(0)
}

pub fn default_tron_account() -> WalletAccountParams {
    tron_account(0)
}

pub fn default_solana_account() -> WalletAccountParams {
    solana_account(0)
}

pub fn default_sui_account() -> WalletAccountParams {
    sui_account(0)
}

pub fn default_aptos_account() -> WalletAccountParams {
    aptos_account(0)
}

pub fn default_xlm_account() -> WalletAccountParams {
    xlm_account(0)
}

pub fn default_bitcoin_mainnet_p2pkh() -> WalletAccountParams {
    bitcoin_mainnet::p2pkh_account(0)
}

pub fn default_bitcoin_mainnet_p2wpkh() -> WalletAccountParams {
    bitcoin_mainnet::p2wpkh_account(0)
}

pub fn default_bitcoin_mainnet_p2wsh() -> WalletAccountParams {
    bitcoin_mainnet::p2wsh_account(0)
}

pub fn default_bitcoin_mainnet_p2tr() -> WalletAccountParams {
    bitcoin_mainnet::p2tr_account(0)
}

pub fn default_bitcoin_mainnet_p2sh() -> WalletAccountParams {
    bitcoin_mainnet::p2sh_account(0)
}

pub fn default_bitcoin_mainnet_accounts() -> [WalletAccountParams; 5] {
    [
        default_bitcoin_mainnet_p2pkh(),
        default_bitcoin_mainnet_p2wpkh(),
        default_bitcoin_mainnet_p2wsh(),
        default_bitcoin_mainnet_p2tr(),
        default_bitcoin_mainnet_p2sh(),
    ]
}

// Dogecoin defaults
pub fn default_doge_mainnet() -> WalletAccountParams {
    dogecoin::mainnet_account(0)
}

pub fn default_doge_testnet() -> WalletAccountParams {
    dogecoin::testnet_account(0)
}

// TON defaults
pub fn default_ton_v3r2() -> WalletAccountParams {
    ton::v3r2_account(0)
}

pub fn default_ton_v4r2() -> WalletAccountParams {
    ton::v4r2_account(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_account {
        ($name:ident, $fn:expr, $path:expr, $curve:expr, $format:expr) => {
            #[test]
            fn $name() {
                let account = $fn(0);
                assert_eq!(account.path, $path);
                assert_eq!(account.curve, $curve);
                assert_eq!(account.address_format, $format);
            }
        };
    }

    test_account!(
        test_ethereum_account_path,
        ethereum_account,
        "m/44'/60'/0'/0/0",
        Curve::CurveSecp256K1,
        AddressFormat::AddressFormatEthereum
    );

    test_account!(
        test_solana_account_path,
        solana_account,
        "m/44'/501'/0'/0'",
        Curve::CurveEd25519,
        AddressFormat::AddressFormatSolana
    );

    #[test]
    fn test_bitcoin_mainnet_accounts() {
        let accounts = bitcoin_mainnet::DEFAULT_ACCOUNTS;
        assert_eq!(accounts.len(), 5);
        assert_eq!(
            accounts[0].address_format,
            AddressFormat::AddressFormatBitcoinMainnetP2Pkh
        );
    }
}
