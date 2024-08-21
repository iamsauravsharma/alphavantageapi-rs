//! Module for exchange currency (both digital & physical currency exchange)
//!
//!  This API returns the realtime exchange rate for any pair of digital
//! currency (e.g., Bitcoin) or physical currency (e.g., USD).
//!
//! You can read about [Exchange][exchange] API and what it returns
//! on alphavantage documentation
//!
//! [exchange]: https://www.alphavantage.co/documentation/#currency-exchnage

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::from_str;
use crate::error::{detect_common_helper_error, Error, Result};

/// Struct used for exchanging currency
#[derive(Default)]
pub struct Exchange {
    real_time: RealtimeExchangeRate,
}

/// Struct Storing Real time Exchange Value
#[derive(Debug, Deserialize, Clone, Default)]
struct RealtimeExchangeRate {
    #[serde(rename = "1. From_Currency Code")]
    from_code: String,
    #[serde(rename = "2. From_Currency Name")]
    from_name: String,
    #[serde(rename = "3. To_Currency Code")]
    to_code: String,
    #[serde(rename = "4. To_Currency Name")]
    to_name: String,
    #[serde(rename = "5. Exchange Rate", deserialize_with = "from_str")]
    rate: f64,
    #[serde(rename = "6. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    time_zone: String,
    #[serde(rename = "8. Bid Price")]
    bid_price: String,
    #[serde(rename = "9. Ask Price")]
    ask_price: String,
}

impl Exchange {
    /// Get Rate for exchange
    #[must_use]
    pub fn rate(&self) -> f64 {
        self.real_time.rate
    }

    /// Get time when exchange rate was last refreshed along with time zone.
    #[must_use]
    pub fn refreshed_time(&self) -> &str {
        &self.real_time.last_refreshed
    }

    /// Return time zone of all data time
    #[must_use]
    pub fn time_zone(&self) -> &str {
        &self.real_time.time_zone
    }

    /// get from code from which exchange is performed
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "EUR").json().await.unwrap();
    ///     let code_from = exchange.code_from();
    ///     assert_eq!(code_from, "BTC");
    /// }
    /// ```
    #[must_use]
    pub fn code_from(&self) -> &str {
        &self.real_time.from_code
    }

    /// get from name from which exchange is performed
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "EUR").json().await.unwrap();
    ///     let name_from = exchange.name_from();
    ///     assert_eq!(name_from, "Bitcoin");
    /// }
    /// ```
    #[must_use]
    pub fn name_from(&self) -> &str {
        &self.real_time.from_name
    }

    /// get to code from exchange
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "EUR").json().await.unwrap();
    ///     let code_to = exchange.code_to();
    ///     assert_eq!(code_to, "EUR");
    /// }
    /// ```
    #[must_use]
    pub fn code_to(&self) -> &str {
        &self.real_time.to_code
    }

    /// get to name from exchange
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "EUR").json().await.unwrap();
    ///     let name_to = exchange.name_to();
    ///     assert_eq!(name_to, "Euro");
    /// }
    /// ```
    #[must_use]
    pub fn name_to(&self) -> &str {
        &self.real_time.to_name
    }

    /// get bid price. Returns None if no bid price
    #[must_use]
    pub fn bid_price(&self) -> Option<f64> {
        self.real_time.bid_price.trim().parse::<f64>().ok()
    }

    /// get ask price. Return None if no ask price
    #[must_use]
    pub fn ask_price(&self) -> Option<f64> {
        self.real_time.ask_price.trim().parse::<f64>().ok()
    }
}

/// Struct used for helping creation of Exchange
#[derive(Debug, Deserialize)]
pub(crate) struct ExchangeHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Realtime Currency Exchange Rate")]
    real_time: Option<RealtimeExchangeRate>,
}

impl ExchangeHelper {
    fn convert(self) -> Result<Exchange> {
        let mut exchange = Exchange::default();
        detect_common_helper_error(self.information, self.error_message, self.note)?;
        if self.real_time.is_none() {
            return Err(Error::EmptyResponse);
        }
        exchange.real_time = self.real_time.unwrap();
        Ok(exchange)
    }
}

/// Builder to create `Exchange`
pub struct ExchangeBuilder<'a> {
    api_client: &'a ApiClient,
    from_currency: &'a str,
    to_currency: &'a str,
}

impl<'a> ExchangeBuilder<'a> {
    crate::json_data_struct!(Exchange, ExchangeHelper);

    /// Create new `ExchangeBuilder` from `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, from_currency: &'a str, to_currency: &'a str) -> Self {
        Self {
            api_client,
            from_currency,
            to_currency,
        }
    }

    fn create_url(&self) -> String {
        format!(
            "query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}",
            self.from_currency, self.to_currency
        )
    }
}
