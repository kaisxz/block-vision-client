use serde::{Deserialize, Serialize};

/// Request parameters for Coin Dex Pools API
///
/// This type encapsulates the request parameters needed to query DEX liquidity pools
/// for a specific coin type. Used by the get_coin_dex_pools endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDexPoolsRequest {
    /// The cointype for the coin to get the information from SuiVision coin page
    pub coin_type: String,
}

/// Represents a single DEX liquidity pool
///
/// This type contains all the relevant information about a liquidity pool
/// including exchange details, pool metrics, and token information.
/// Used as the response type for the get_coin_dex_pools endpoint.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinDexPoolsResponse {
    /// The name of the decentralized exchange (DEX) where the pool is located
    /// Example: "momentum"
    pub dex: String,

    /// A URL linking to the pool's detail page on the corresponding DEX
    pub link: String,

    /// The unique identifier (address) of the liquidity pool
    pub pool_id: String,

    /// The total token balance in the pool (typically in base units, e.g., token amount as stringified decimal)
    pub balance: String,

    /// The current token price in the pool, quoted against the other asset
    pub price: String,

    /// List of token type tags in this pool (usually a token pair)
    pub coin_list: Vec<String>,

    /// Total Value Locked in the pool, denominated in USD
    pub tvl: String,

    /// Estimated Annual Percentage Rate for liquidity providers in this pool (optional, may be empty)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
}
