//! Module for return earning per share for a company
//!
//! This API returns the annual and quarterly earnings (EPS) for the company of
//! interest. Quarterly data also includes analyst estimates and surprise
//! metrics.

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::{from_none_str, from_str};
use crate::error::{Error, Result};

/// Struct to store information of annual earning
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Annual {
    #[serde(rename = "fiscalDateEnding")]
    fiscal_date_ending: String,
    #[serde(rename = "reportedEPS", deserialize_with = "from_str")]
    reported_eps: f64,
}

impl Annual {
    /// Return annual earning fiscal date ending
    #[must_use]
    pub fn fiscal_date_ending(&self) -> &str {
        &self.fiscal_date_ending
    }

    /// Return reported eps for annual earning
    #[must_use]
    pub fn reported_eps(&self) -> f64 {
        self.reported_eps
    }
}

/// Struct to store information of quarterly earning
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Quarterly {
    #[serde(rename = "fiscalDateEnding")]
    fiscal_date_ending: String,
    #[serde(rename = "reportedDate")]
    reported_date: String,
    #[serde(rename = "reportedEPS", deserialize_with = "from_none_str")]
    reported_eps: Option<f64>,
    #[serde(rename = "estimatedEPS", deserialize_with = "from_str")]
    estimated_eps: f64,
    #[serde(rename = "surprise", deserialize_with = "from_none_str")]
    surprise: Option<f64>,
    #[serde(rename = "surprisePercentage", deserialize_with = "from_none_str")]
    surprise_percentage: Option<f64>,
}

impl Quarterly {
    /// Return fiscal date ending quarterly earning
    #[must_use]
    pub fn fiscal_date_ending(&self) -> &str {
        &self.fiscal_date_ending
    }

    /// Return reported date for quarterly earning
    #[must_use]
    pub fn reported_date(&self) -> &str {
        &self.reported_date
    }

    /// Return reported eps of symbol for quarter. Return None if api return
    /// none
    #[must_use]
    pub fn reported_eps(&self) -> Option<f64> {
        self.reported_eps
    }

    /// Return Estimated eps of symbol for quarter
    #[must_use]
    pub fn estimated_eps(&self) -> f64 {
        self.estimated_eps
    }

    /// Return value of surprise. return None if api return none
    #[must_use]
    pub fn surprise(&self) -> Option<f64> {
        self.surprise
    }

    /// Return surprise percentage for symbol quarterly earning. Return None if
    /// api return None
    #[must_use]
    pub fn surprise_percentage(&self) -> Option<f64> {
        self.surprise_percentage
    }
}

/// Struct to store earning for symbol
#[derive(Debug, Default)]
pub struct Earning {
    symbol: String,
    annual_earning: Vec<Annual>,
    quarterly_earning: Vec<Quarterly>,
}

impl Earning {
    /// Return symbol of company
    ///
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
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return Annual earning list for symbol
    #[must_use]
    pub fn annual_earning(&self) -> &Vec<Annual> {
        &self.annual_earning
    }

    /// Return quarterly earning for symbol
    #[must_use]
    pub fn quarterly_earning(&self) -> &Vec<Quarterly> {
        &self.quarterly_earning
    }
}

/// Struct used for creating earning
#[derive(Debug, Deserialize)]
pub(crate) struct EarningHelper {
    #[serde(rename = "symbol")]
    symbol: Option<String>,
    #[serde(rename = "annualEarnings")]
    annual_earning: Option<Vec<Annual>>,
    #[serde(rename = "quarterlyEarnings")]
    quarterly_earning: Option<Vec<Quarterly>>,
}
impl EarningHelper {
    /// Function which convert `EarningHelper` to
    /// `Earning`
    fn convert(self) -> Result<Earning> {
        let mut earning = Earning::default();
        if self.symbol.is_none()
            || self.annual_earning.is_none()
            || self.quarterly_earning.is_none()
        {
            return Err(Error::EmptyResponse);
        }
        earning.symbol = self.symbol.unwrap();
        earning.annual_earning = self.annual_earning.unwrap();
        earning.quarterly_earning = self.quarterly_earning.unwrap();
        Ok(earning)
    }
}

/// Builder to help create Earning
pub struct EarningBuilder<'a> {
    api_client: &'a ApiClient,
    symbol: &'a str,
}

impl<'a> EarningBuilder<'a> {
    crate::json_data_struct!(Earning, EarningHelper);

    /// Create new Earning Builder with help of `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, symbol: &'a str) -> Self {
        Self { api_client, symbol }
    }

    fn create_url(&self) -> String {
        format!("query?function=EARNINGS&symbol={}", self.symbol)
    }
}
