//! Module for searching specific symbol or companies
//!
//! Looking for some specific symbols or companies? Trying to build a search box
//! similar to the one below?
//!
//! You can read about [Symbol][symbol_search] API and what it returns
//! on alphavantage documentation
//!
//! [symbol_search]: https://www.alphavantage.co/documentation/#symbolsearch

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::from_str;
use crate::error::{Error, Result};

/// Struct which stores matches data for search keyword
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Match {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    stock_type: String,
    #[serde(rename = "4. region")]
    region: String,
    #[serde(rename = "5. marketOpen")]
    market_open: String,
    #[serde(rename = "6. marketClose")]
    market_close: String,
    #[serde(rename = "7. timezone")]
    time_zone: String,
    #[serde(rename = "8. currency")]
    currency: String,
    #[serde(rename = "9. matchScore", deserialize_with = "from_str")]
    match_score: f64,
}

impl Match {
    /// Return symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let symbol = search.matches()[0].symbol();
    ///     assert_eq!(symbol, "BA");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return name for symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let name = search.matches()[0].name();
    ///     assert_eq!(name, "Boeing Company");
    /// }
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return stock type
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let stock_type = search.matches()[0].stock_type();
    ///     assert_eq!(stock_type, "Equity");
    /// }
    #[must_use]
    pub fn stock_type(&self) -> &str {
        &self.stock_type
    }

    /// Return region of search data
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let region = search.matches()[0].region();
    ///     assert_eq!(region, "United States");
    /// }
    #[must_use]
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return market open time
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let market_open = search.matches()[0].market_open();
    ///     assert_eq!(market_open, "09:30");
    /// }
    #[must_use]
    pub fn market_open(&self) -> &str {
        &self.market_open
    }

    /// Return market close time
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let market_close = search.matches()[0].market_close();
    ///     assert_eq!(market_close, "16:00");
    /// }
    #[must_use]
    pub fn market_close(&self) -> &str {
        &self.market_close
    }

    /// Return time zone of symbol
    #[must_use]
    pub fn time_zone(&self) -> &str {
        &self.time_zone
    }

    /// Return currency
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let currency = search.matches()[0].currency();
    ///     assert_eq!(currency, "USD");
    /// }
    #[must_use]
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Return match score
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let match_score = search.matches()[0].match_score();
    ///     assert_eq!(match_score, 1.0);
    /// }
    #[must_use]
    pub fn match_score(&self) -> f64 {
        self.match_score
    }
}

/// struct for storing search method data
#[derive(Default)]
pub struct Search {
    matches: Vec<Match>,
}

impl Search {
    /// Return result of search
    #[must_use]
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }
}

/// struct for helping creation of search struct
#[derive(Debug, Deserialize)]
pub(crate) struct SearchHelper {
    #[serde(rename = "bestMatches")]
    matches: Option<Vec<Match>>,
}

impl SearchHelper {
    fn convert(self) -> Result<Search> {
        let mut search = Search::default();
        if self.matches.is_none() {
            return Err(Error::EmptyResponse);
        }
        search.matches = self.matches.unwrap();
        Ok(search)
    }
}

/// Builder to create new `Search`
pub struct SearchBuilder<'a> {
    api_client: &'a ApiClient,
    keywords: &'a str,
}

impl<'a> SearchBuilder<'a> {
    crate::json_data_struct!(Search, SearchHelper);

    /// Create new `SearchBuilder` from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, keywords: &'a str) -> Self {
        Self {
            api_client,
            keywords,
        }
    }

    fn create_url(&self) -> String {
        format!("query?function=SYMBOL_SEARCH&keywords={}", self.keywords)
    }
}
