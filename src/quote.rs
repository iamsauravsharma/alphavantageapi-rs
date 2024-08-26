//! Module for returning latest price and volume information
//!
//! A lightweight alternative to the time series APIs, this service returns the
//! latest price and volume information for a security of your choice.
//!
//! You can read about [Quote][quote] API and what it returns
//! on alphavantage documentation
//!
//! [quote]: https://www.alphavantage.co/documentation/#latestprice

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::{from_str, percent_f64};
use crate::error::{Error, Result};

/// Struct storing Global Quote Value
#[derive(Debug, Deserialize, Clone, Default)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "03. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "04. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "05. price", deserialize_with = "from_str")]
    price: f64,
    #[serde(rename = "06. volume", deserialize_with = "from_str")]
    volume: u64,
    #[serde(rename = "07. latest trading day")]
    last_day: String,
    #[serde(rename = "08. previous close", deserialize_with = "from_str")]
    previous_close: f64,
    #[serde(rename = "09. change", deserialize_with = "from_str")]
    change: f64,
    #[serde(rename = "10. change percent", deserialize_with = "percent_f64")]
    change_percent: f64,
}

/// Struct for storing Quote related information
#[derive(Default)]
pub struct Quote {
    global_quote: GlobalQuote,
}

impl Quote {
    /// return open value
    #[must_use]
    pub fn open(&self) -> f64 {
        self.global_quote.open
    }

    /// return high value
    #[must_use]
    pub fn high(&self) -> f64 {
        self.global_quote.high
    }

    /// return low value
    #[must_use]
    pub fn low(&self) -> f64 {
        self.global_quote.low
    }

    /// return price value
    #[must_use]
    pub fn price(&self) -> f64 {
        self.global_quote.price
    }

    /// return volume
    #[must_use]
    pub fn volume(&self) -> u64 {
        self.global_quote.volume
    }

    /// return previous
    #[must_use]
    pub fn previous(&self) -> f64 {
        self.global_quote.previous_close
    }

    /// return change
    #[must_use]
    pub fn change(&self) -> f64 {
        self.global_quote.change
    }

    /// return change percent
    #[must_use]
    pub fn change_percent(&self) -> f64 {
        self.global_quote.change_percent
    }

    /// get last trading day
    #[must_use]
    pub fn last_trading(&self) -> &str {
        &self.global_quote.last_day
    }

    /// get symbol
    ///
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
    pub fn symbol(&self) -> &str {
        &self.global_quote.symbol
    }
}

/// Struct for helping creation of Quote
#[derive(Debug, Deserialize)]
pub(crate) struct QuoteHelper {
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
}

impl QuoteHelper {
    fn convert(self) -> Result<Quote> {
        let mut quote = Quote::default();
        if self.global_quote.is_none() {
            return Err(Error::EmptyResponse);
        }
        quote.global_quote = self.global_quote.unwrap();
        Ok(quote)
    }
}

/// Builder to create `Quote`
pub struct QuoteBuilder<'a> {
    api_client: &'a ApiClient,
    symbol: &'a str,
}

impl<'a> QuoteBuilder<'a> {
    crate::json_data_struct!(Quote, QuoteHelper);

    /// Create new `QuoteBuilder` from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, symbol: &'a str) -> Self {
        Self { api_client, symbol }
    }

    fn create_url(&self) -> String {
        format!("query?function=GLOBAL_QUOTE&symbol={}", self.symbol)
    }
}
