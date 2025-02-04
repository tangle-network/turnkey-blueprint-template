# <h1 align="center">Turnkey Automation Tangle Blueprint 🔐</h1>

## 📚 Overview

This Tangle Blueprint provides automated wallet infrastructure services using Turnkey. Blueprints are specifications for <abbr title="Actively Validated Services">AVS</abbr>s on the Tangle Network. An AVS is an off-chain service that runs arbitrary computations for a user-specified period of time.

This blueprint includes two Rust crates for Turnkey integration:

- [turnkey-rs](turnkey-rs/README.md) - High-level Rust SDK for Turnkey wallet automation
- [turnkey-api](turnkey-api/README.md) - Generated API client from OpenAPI spec

The blueprint enables automated wallet operations such as:

- Wallet creation across multiple chains
- Transaction signing and broadcasting
- Key management and rotation
- Automated security policies
- Custom wallet naming and derivation paths

## 🚀 Features

- Multi-chain wallet automation
  - Ethereum (Secp256k1)
  - Bitcoin (P2PKH, P2SH, P2WPKH, P2WSH, P2TR)
  - Solana (Ed25519)
- Automated wallet creation with configurable parameters
- Type-safe API integration
- Secure key management
- Customizable automation policies

## 📋 Prerequisites

Before you can run this project, you will need to have the following software installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh)
- [Turnkey Account](https://turnkey.com) with API credentials

You will also need to install [cargo-tangle](https://crates.io/crates/cargo-tangle), our CLI tool for creating and deploying Tangle Blueprints.

## ⭐ Getting Started

1. Copy the example environment file and set up your Turnkey credentials:

```sh
cp .env.example .env
# Then edit .env with your credentials:

export TURNKEY_API_PUBLIC_KEY=your_public_key
export TURNKEY_API_PRIVATE_KEY=your_private_key
export TURNKEY_ORGANIZATION_ID=your_org_id
```

The required environment variables are:

- `TURNKEY_API_PUBLIC_KEY`: Your API public key from Turnkey dashboard
- `TURNKEY_API_PRIVATE_KEY`: Your API private key from Turnkey dashboard
- `TURNKEY_ORGANIZATION_ID`: Your organization ID from Turnkey dashboard

2. Create a new project:

```sh
cargo tangle blueprint create --name <project-name>
```

3. Follow the instructions to create a new project.

## 🛠️ Development

Before building or running the project, make sure your environment variables are set up correctly in `.env`.

Build the project:

```sh
cargo build
```
