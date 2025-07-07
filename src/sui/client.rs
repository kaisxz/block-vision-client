use crate::{
    error::ClientError,
    sui::{extensions::{request_builder::RequestBuilderExt, response::ResponseExt}, types::{
        coin_detail::CoinDetailResponse, coin_dex_pools::CoinDexPoolsResponse, ApiResponse
    }},
};
use http::StatusCode;
use reqwest::Client;
use secrecy::SecretString;
use url::Url;

const BASE_URL: &str = "https://api.blockvision.org/v2/sui/";

pub struct SuiClient {
    http: Client,
    api_key: SecretString,
    base_url: Url,
}

impl SuiClient {
    pub fn new(api_key: SecretString) -> Self {
        Self {
            http: Client::new(),
            api_key,
            base_url: Url::parse(BASE_URL).unwrap(),
        }
    }

    /// Retrieve a list of decentralized exchange (DEX) liquidity pools associated with a specific coin.
    ///
    /// This endpoint is useful for discovering which pools a coin is actively traded in, as well as
    /// retrieving relevant pool-level data such as token pair, liquidity, volume, and price.
    ///
    /// # Arguments
    /// * `coin_type` - The cointype for the coin to get the information from SuiVision coin page
    ///
    /// # Returns
    /// * `Result<CoinDexPoolsResponse, ClientError>` - List of DEX pools for the specified coin
    pub async fn get_coin_dex_pools(
        &self,
        coin_type: &str,
    ) -> Result<CoinDexPoolsResponse, ClientError> {
        let url = self.base_url.join("coin/dex/pools").unwrap();
        // Build query parameters
        let params = vec![("coinType", coin_type)];

        let resp = self
            .http
            .get(url)
            .with_default_headers(&self.api_key)
            .query(&params)
            .send()
            .await?;

        let parsed = resp.json_or_err::<ApiResponse<CoinDexPoolsResponse>>().await?;

        if !parsed.code.is_success() {
            return Err(ClientError::Other(parsed.message));
        }

        Ok(parsed.result.unwrap())
    }

    /// Retrieve detailed coin information including metadata, price, supply, and verification status.
    ///
    /// This endpoint provides comprehensive information about a coin including its metadata,
    /// market data, supply information, and verification status from SuiVision.
    ///
    /// Reference: https://docs.blockvision.org/reference/retrieve-coin-detail
    ///
    /// # Arguments
    /// * `coin_type` - The cointype for the coin to get the information from SuiVision coin page
    ///
    /// # Returns
    /// * `Result<CoinDetailResponse, ClientError>` - Detailed coin information
    pub async fn get_coin_detail(
        &self,
        coin_type: &str,
    ) -> Result<Option<CoinDetailResponse>, ClientError> {
        let url = self.base_url.join("coin/detail").unwrap();
        // Build query parameters
        let params = vec![("coinType", coin_type)];

        let resp = self
            .http
            .get(url)
            .with_default_headers(&self.api_key)
            .query(&params)
            .send()
            .await?;

        let parsed = resp
            .json_or_err::<ApiResponse<CoinDetailResponse>>()
            .await
            .unwrap();

        if !parsed.code.is_success() {
            return Err(ClientError::Other(parsed.message));
        }

        Ok(parsed.result)
    }
}
