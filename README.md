# BlockVision Client

[![Crates.io](https://img.shields.io/crates/v/block-vision-client)](https://crates.io/crates/block-vision-client)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-edition-orange.svg)](https://www.rust-lang.org/)

A high-performance, type-safe Rust client for the BlockVision API, providing seamless access to blockchain data for Sui and Monad networks. Built with modern Rust practices, featuring async/await support, comprehensive error handling, and a modular architecture.

## 🚀 Features

- **Multi-Network Support**: Native support for Sui and Monad blockchains
- **Type-Safe API**: Full type safety with comprehensive response models
- **Async/Await**: Built on Tokio for high-performance async operations
- **Modular Architecture**: Feature-gated modules for different blockchain networks
- **Error Handling**: Comprehensive error types with detailed error messages
- **Security**: Secure API key handling with secrecy crate
- **Extensible**: Easy to extend for additional blockchain networks

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
block-vision-client = "0.1.0"
```

### Feature Flags

The crate supports feature flags to enable specific blockchain networks:

- `sui` (default): Enables Sui blockchain support
- `monad` (default): Enables Monad blockchain support

```toml
# Enable only Sui support
block-vision-client = { version = "0.1.0", default-features = false, features = ["sui"] }

# Enable only Monad support
block-vision-client = { version = "0.1.0", default-features = false, features = ["monad"] }
```

## 🔧 Usage

### Basic Setup

```rust
use block_vision_client::sui::prelude::*;
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let api_key = SecretString::new("your-api-key-here".to_string());
    let client = BlockVisionSuiClient::new(api_key);

    // Your API calls here...
    Ok(())
}
```

### Sui Blockchain Examples

#### Get Coin Details

```rust
use block_vision_client::sui::prelude::*;
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = SecretString::new("your-api-key-here".to_string());
    let client = BlockVisionSuiClient::new(api_key);

    // Get detailed information about a specific coin
    let coin_type = "0x2::sui::SUI";
    match client.get_coin_detail(coin_type).await? {
        Some(coin_detail) => {
            println!("Coin Name: {}", coin_detail.name);
            println!("Symbol: {}", coin_detail.symbol);
            println!("Price: ${}", coin_detail.price);
            println!("Market Cap: ${}", coin_detail.market_cap);
            println!("Holders: {}", coin_detail.holders);
            println!("Verified: {}", coin_detail.verified);
        }
        None => println!("Coin not found"),
    }

    Ok(())
}
```

#### Get DEX Pools for a Coin

```rust
use block_vision_client::sui::prelude::*;
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = SecretString::new("your-api-key-here".to_string());
    let client = BlockVisionSuiClient::new(api_key);

    // Get DEX pools for a specific coin
    let coin_type = "0x2::sui::SUI";
    let pools = client.get_coin_dex_pools(coin_type).await?;
    
    for pool in pools {
        println!("DEX: {}", pool.dex);
        println!("Pool ID: {}", pool.pool_id);
        println!("TVL: ${}", pool.tvl);
        println!("Price: {}", pool.price);
        println!("APR: {:?}", pool.apr);
        println!("---");
    }

    Ok(())
}
```

### Error Handling

The client provides comprehensive error handling:

```rust
use block_vision_client::{sui::prelude::*, error::ClientError};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let api_key = SecretString::new("your-api-key-here".to_string());
    let client = BlockVisionSuiClient::new(api_key);

    match client.get_coin_detail("0x2::sui::SUI").await {
        Ok(Some(coin)) => println!("Found coin: {}", coin.name),
        Ok(None) => println!("Coin not found"),
        Err(ClientError::Http(e)) => eprintln!("HTTP error: {}", e),
        Err(ClientError::Json(e)) => eprintln!("JSON parsing error: {}", e),
        Err(ClientError::Other(msg)) => eprintln!("API error: {}", msg),
    }

    Ok(())
}
```

## 📚 API Reference

### Sui Client

#### `SuiClient::new(api_key: SecretString) -> Self`

Creates a new Sui client instance.

#### `get_coin_detail(coin_type: &str) -> Result<Option<CoinDetailResponse>, ClientError>`

Retrieves detailed coin information including metadata, price, supply, and verification status.

**Parameters:**
- `coin_type`: The coin type identifier (e.g., "0x2::sui::SUI")

**Returns:**
- `CoinDetailResponse` with comprehensive coin information

#### `get_coin_dex_pools(coin_type: &str) -> Result<CoinDexPoolsResponse, ClientError>`

Retrieves a list of DEX liquidity pools associated with a specific coin.

**Parameters:**
- `coin_type`: The coin type identifier

**Returns:**
- `CoinDexPoolsResponse` with pool information

### Response Types

#### `CoinDetailResponse`

```rust
pub struct CoinDetailResponse {
    pub name: String,                    // Coin's full name
    pub symbol: String,                  // Coin ticker symbol
    pub decimals: i32,                   // Token decimals
    pub logo: String,                    // Logo URL
    pub price: String,                   // Current USD price
    pub price_change_percentage_24h: String, // 24h price change
    pub holders: i32,                    // Number of token holders
    pub market_cap: String,              // Market capitalization
    pub website: String,                 // Project website
    pub creator: String,                 // Creator address
    pub created_time: i64,               // Creation timestamp
    pub verified: bool,                  // Verification status
    pub circulating: String,             // Circulating supply
    pub scam_flag: u8,                   // Scam flag indicator
}
```

#### `CoinDexPoolsResponse`

```rust
pub struct CoinDexPoolsResponse {
    pub dex: String,                     // DEX name
    pub link: String,                    // Pool detail URL
    pub pool_id: String,                 // Pool identifier
    pub balance: String,                 // Pool token balance
    pub price: String,                   // Token price in pool
    pub coin_list: Vec<String>,          // Pool token list
    pub tvl: String,                     // Total Value Locked
    pub apr: Option<String>,             // Annual Percentage Rate
}
```

## 🔒 Security

- API keys are handled securely using the `secrecy` crate
- All sensitive data is wrapped in `SecretString` to prevent accidental logging
- HTTPS is enforced for all API communications

## 🏗️ Architecture

The project follows a modular architecture with feature-gated modules:

```
src/
├── lib.rs              # Main library entry point
├── error.rs            # Error types and handling
├── sui/                # Sui blockchain support
│   ├── client.rs       # Sui API client
│   ├── prelude.rs      # Public exports
│   ├── types/          # Response and request types
│   └── extensions/     # HTTP extensions
└── monad/              # Monad blockchain support
    ├── client.rs       # Monad API client
    ├── types/          # Response and request types
    └── websocket.rs    # WebSocket support
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run tests with specific features:

```bash
cargo test --features sui
cargo test --features monad
```

## 📝 Examples

See the [examples directory](examples/) for complete usage examples.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/kaisxz/block-vision-client.git
   cd block-vision-client
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [BlockVision API Documentation](https://docs.blockvision.org/)
- [Sui Blockchain](https://sui.io/)
- [Monad Blockchain](https://monad.xyz/)
- [Crates.io](https://crates.io/crates/block-vision-client)
- [GitHub Repository](https://github.com/kaisxz/block-vision-client)

## ⚡ Performance

- Built with Tokio for high-performance async operations
- Efficient HTTP client with connection pooling
- Minimal memory footprint with zero-copy deserialization
- Type-safe operations reduce runtime errors

## 🛠️ Dependencies

- **reqwest**: HTTP client with JSON support
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **thiserror**: Error handling
- **secrecy**: Secure string handling
- **tokio-tungstenite**: WebSocket support
- **async-trait**: Async trait support

---

**Note**: This client requires a valid BlockVision API key. You can obtain one by signing up at [BlockVision](https://blockvision.org/).
