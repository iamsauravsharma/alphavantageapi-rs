use async_trait::async_trait;

use crate::error::{Error, Result};

#[async_trait]
/// Trait which can be implemented for all common library client for getting
/// output from server
///
/// reqwest client is supported with feature flag. If you prefer alternate http
/// client you can add support by implementing `HttpClient` trait for client.
/// Some example of other client which can be used are surf and isahc client or
/// other reqwest version
pub trait HttpClient {
    /// AlphaVantage provider output function which provides one field path
    /// where get GET request needs to be performed
    async fn get_alpha_vantage_provider_output(&self, path: &str) -> Result<String>;

    /// RapidAPI provider function which provides two field path and api_key.
    /// Path needs to be set along with header x-rapidapi-host as
    /// alpha-vantage.p.rapidapi.com and header x-rapidapi-key same as
    /// api_key field
    async fn get_rapid_api_provider_output(&self, path: &str, api_key: &str) -> Result<String>;
}

#[cfg(feature = "reqwest-client")]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn get_alpha_vantage_provider_output(&self, path: &str) -> Result<String> {
        self.get(path)
            .send()
            .await
            .map_err(|_| Error::GetRequestFailed)?
            .text()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }

    async fn get_rapid_api_provider_output(&self, path: &str, api_key: &str) -> Result<String> {
        self.get(path)
            .header("x-rapidapi-host", "alpha-vantage.p.rapidapi.com")
            .header("x-rapidapi-key", api_key)
            .send()
            .await
            .map_err(|_| Error::GetRequestFailed)?
            .text()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }
}
