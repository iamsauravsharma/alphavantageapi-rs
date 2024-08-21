use serde::de::DeserializeOwned;

use crate::client::HttpClient;
use crate::crypto::{CryptoBuilder, CryptoFunction};
use crate::custom::CustomBuilder;
use crate::earning::EarningBuilder;
use crate::economic_indicator::EconomicIndicatorBuilder;
use crate::error::{Error, Result};
use crate::exchange::ExchangeBuilder;
use crate::forex::{ForexBuilder, ForexFunction};
use crate::quote::QuoteBuilder;
use crate::search::SearchBuilder;
use crate::stock_time::{StockFunction, TimeSeriesBuilder};
use crate::technical_indicator::{TechnicalIndicatorBuilder, TechnicalIndicatorInterval};

const BASE_URL: &str = "https://www.alphavantage.co/";
const RAPID_API_BASE_URL: &str = "https://alpha-vantage.p.rapidapi.com/query";

/// Provider for alpha vantage API
pub enum Provider {
    /// Use alphavantage API provider
    AlphaVantage,
    /// User `RapidAPI` as provider
    RapidAPI,
}

/// Struct for initializing client which contains different method for API call
pub struct ApiClient {
    api: String,
    client: Box<dyn HttpClient + Send + Sync>,
    provider: Provider,
}

impl ApiClient {
    /// Method for initializing `ApiClient` struct using  user
    /// provided client and alphavantage.co provider
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = ApiClient::set_api("some_key", reqwest::Client::new());
    /// ```
    #[must_use]
    pub fn set_api<S, T>(api: S, client: T) -> Self
    where
        S: Into<String>,
        T: HttpClient + 'static + Send + Sync,
    {
        Self {
            api: api.into(),
            client: Box::new(client),
            provider: Provider::AlphaVantage,
        }
    }

    /// Method for initializing `ApiClient` struct using user
    /// provided client and `RapidAPI` API provider
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = ApiClient::set_api("some_key", reqwest::Client::new());
    /// ```
    #[must_use]
    pub fn set_rapid_api<S, T>(api: S, client: T) -> Self
    where
        S: Into<String>,
        T: HttpClient + 'static + Send + Sync,
    {
        Self {
            api: api.into(),
            client: Box::new(client),
            provider: Provider::RapidAPI,
        }
    }

    /// Method to get api key
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = alpha_vantage::api::ApiClient::set_api("some_key", reqwest::Client::new());
    /// assert_eq!(api.get_api_key(), "some_key");
    /// ```
    #[must_use]
    pub fn get_api_key(&self) -> &str {
        &self.api
    }

    // Get json from api endpoint and create struct
    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let string_output = match &self.provider {
            Provider::AlphaVantage => {
                self.client
                    .get_alpha_vantage_provider_output(&format!(
                        "{BASE_URL}{path}&apikey={}",
                        self.api
                    ))
                    .await
            }
            Provider::RapidAPI => {
                self.client
                    .get_rapid_api_provider_output(
                        &format!("{RAPID_API_BASE_URL}{path}"),
                        &self.api,
                    )
                    .await
            }
        }?;
        serde_json::from_str(&string_output).map_err(|_| Error::DecodeJsonToStruct)
    }

    /// Crypto method for calling cryptography function with help of
    /// `CryptoBuilder`
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "EUR")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(crypto.digital_code(), "BTC");
    ///     assert_eq!(crypto.digital_name(), "Bitcoin");
    ///     assert_eq!(crypto.market_code(), "EUR");
    ///     assert_eq!(crypto.market_name(), "Euro");
    /// }
    /// ```
    #[must_use]
    pub fn crypto<'a>(
        &'a self,
        function: CryptoFunction,
        symbol: &'a str,
        market: &'a str,
    ) -> CryptoBuilder<'a> {
        CryptoBuilder::new(self, function, symbol, market)
    }

    /// Method for calling custom function not implemented currently in library
    /// using `CustomBuilder`
    #[must_use]
    pub fn custom<'a>(&'a self, function: &'a str) -> CustomBuilder<'a> {
        CustomBuilder::new(self, function)
    }

    /// Method for returning `EarningBuilder` for earning API
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let earning = api.earning("IBM").json().await.unwrap();
    ///     let symbol = earning.symbol();
    ///     assert_eq!(symbol, "IBM");
    /// }
    /// ```
    #[must_use]
    pub fn earning<'a>(&'a self, symbol: &'a str) -> EarningBuilder<'a> {
        EarningBuilder::new(self, symbol)
    }

    /// Method for economic indicator builder
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let economic = api
    ///         .economic_indicator("REAL_GDP_PER_CAPITA")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(economic.interval(), "quarterly");
    /// }
    /// ```
    #[must_use]
    pub fn economic_indicator<'a>(&'a self, function: &'a str) -> EconomicIndicatorBuilder<'a> {
        EconomicIndicatorBuilder::new(self, function)
    }

    /// Method for creating `ExchangeBuilder` for exchanging currency value from
    /// one currency to another currency.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "EUR").json().await.unwrap();
    ///     assert_eq!(exchange.name_from(), "Bitcoin");
    ///     assert_eq!(exchange.code_from(), "BTC");
    ///     assert_eq!(exchange.name_to(), "Euro");
    ///     assert_eq!(exchange.code_to(), "EUR");
    /// }
    /// ```
    #[must_use]
    pub fn exchange<'a>(
        &'a self,
        from_currency: &'a str,
        to_currency: &'a str,
    ) -> ExchangeBuilder<'a> {
        ExchangeBuilder::new(self, from_currency, to_currency)
    }

    /// Method for creating `ForexBuilder` for `Forex` API
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::Weekly, "EUR", "USD")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(forex.symbol_from(), "EUR");
    ///     assert_eq!(forex.symbol_to(), "USD");
    ///     assert!(forex.interval().is_none());
    /// }
    /// ```
    #[must_use]
    pub fn forex<'a>(
        &'a self,
        function: ForexFunction,
        from_symbol: &'a str,
        to_symbol: &'a str,
    ) -> ForexBuilder<'a> {
        ForexBuilder::new(self, function, from_symbol, to_symbol)
    }

    /// Method for creating `QuoteBuilder` from `APIClient`
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let quote = api.quote("MSFT").json().await.unwrap();
    ///     let symbol = quote.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    #[must_use]
    pub fn quote<'a>(&'a self, symbol: &'a str) -> QuoteBuilder<'a> {
        QuoteBuilder::new(self, symbol)
    }

    /// Method for creating search builder
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let first_search_match = &search.matches()[0];
    ///     assert_eq!(first_search_match.symbol(), "BA");
    ///     assert_eq!(first_search_match.name(), "Boeing Company");
    ///     assert_eq!(first_search_match.stock_type(), "Equity");
    ///     assert_eq!(first_search_match.region(), "United States");
    ///     assert_eq!(first_search_match.currency(), "USD");
    ///     assert_eq!(first_search_match.match_score(), 1.0);
    /// }
    /// ```
    #[must_use]
    pub fn search<'a>(&'a self, keywords: &'a str) -> SearchBuilder<'a> {
        SearchBuilder::new(self, keywords)
    }

    /// Method for creating Stock time Builder from `APIClient`
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock = api
    ///         .stock_time(alpha_vantage::stock_time::StockFunction::Weekly, "MSFT")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(stock.symbol(), "MSFT");
    ///     assert!(stock.interval().is_none());
    /// }
    /// ```
    #[must_use]
    pub fn stock_time<'a>(
        &'a self,
        function: StockFunction,
        symbol: &'a str,
    ) -> TimeSeriesBuilder<'a> {
        TimeSeriesBuilder::new(self, function, symbol)
    }

    /// Method for technical indicator builder
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let technical = api
    ///         .technical_indicator(
    ///             "MAMA",
    ///             "IBM",
    ///             alpha_vantage::technical_indicator::TechnicalIndicatorInterval::Daily,
    ///         )
    ///         .series_type("close")
    ///         .extra_param("fastlimit", 0.02)
    ///         .json()
    ///         .await;
    ///     assert!(technical.is_ok());
    /// }
    /// ```
    #[must_use]
    pub fn technical_indicator<'a>(
        &'a self,
        function: &'a str,
        symbol: &'a str,
        interval: TechnicalIndicatorInterval,
    ) -> TechnicalIndicatorBuilder<'a> {
        TechnicalIndicatorBuilder::new(self, function, symbol, interval)
    }
}

/// Enum for declaring output size of API call
#[derive(Clone)]
pub enum OutputSize {
    /// Return latest top 100 points recommended if no historical data is
    /// required and decreases api json sizes
    Compact,
    /// Returns full api data points recommended if a full historical data is
    /// required
    Full,
}

/// Enum for declaring interval for intraday time series
#[derive(Clone)]
pub enum TimeSeriesInterval {
    /// 1 min interval
    OneMin,
    /// 5 min interval
    FiveMin,
    /// 15 min interval
    FifteenMin,
    /// 30 min interval
    ThirtyMin,
    /// 60 min interval
    SixtyMin,
}
