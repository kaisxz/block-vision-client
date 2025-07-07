use serde::{Deserialize, Serialize};

/// Request parameters for Coin Detail API
///
/// This type encapsulates the request parameters needed to query detailed information
/// about a specific coin type. Used by the get_coin_detail endpoint.
///
/// Reference: https://docs.blockvision.org/reference/retrieve-coin-detail
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDetailRequest {
    /// The cointype for the coin to get the information from SuiVision coin page
    pub coin_type: String,
}

/// Represents detailed coin information including metadata, price, supply, and verification
///
/// This type contains comprehensive information about a coin including its metadata,
/// market data, supply information, and verification status.
/// Used as the response type for the get_coin_detail endpoint.
///
/// Reference: https://docs.blockvision.org/reference/retrieve-coin-detail
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinDetailResponse {
    /// The coin's full name
    pub name: String,

    /// The standardized coin tick from its metadata
    pub symbol: String,

    /// The coin decimals from its metadata
    pub decimals: i32,

    /// URL to the coin's logo
    pub logo: String,

    /// The current market price of the coin in USD
    pub price: String,

    /// The 24-hour percentage change in the coin's price
    #[serde(rename = "priceChangePercentage24H")]
    pub price_change_percentage_24h: String,

    /// Count of unique addresses currently holding this token, indicating its distribution among users
    pub holders: i32,

    /// The total market capitalization, calculated as price * totalSupply, representing the token's market value
    pub market_cap: String,

    /// The website or X's account associated with the given coin, returns "" if empty
    pub website: String,

    /// A 32 Byte address with '0x' as prefix. The account address of the coin creator
    pub creator: String,

    /// The timestamp of the coin created by the publisher in milliseconds
    pub created_time: i64,

    /// Returns true if this coin is verified at SuiVision explorer
    pub verified: bool,

    /// The aggregate quantity of tokens issued, representing maximum circulating supply
    pub circulating: String,

    /// Returns true if this coin is labelled as scam at SuiVision explorer
    pub scam_flag: u8,
}
