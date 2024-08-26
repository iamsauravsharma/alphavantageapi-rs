//! Module for Forex realtime and historical data
//!
//! APIs under this section provide a wide range of data feed for realtime and
//! historical forex (FX) rates.
//!
//! You can read about [Forex][forex] API and what it returns
//! on alphavantage documentation
//!
//! [forex]: https://www.alphavantage.co/documentation/#fx

use std::cmp;
use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{ApiClient, OutputSize, TimeSeriesInterval};
use crate::deserialize::from_str;
use crate::error::{Error, Result};
use crate::vec_trait::FindData;

/// Struct used to store metadata value
#[derive(Debug, Clone, Default)]
struct MetaData {
    information: String,
    from_symbol: String,
    to_symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

/// Struct to store Data value
#[derive(Default, Debug, Clone)]
pub struct Data {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl Data {
    /// Return time for data
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return open value
    #[must_use]
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Return high value
    #[must_use]
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Return low value
    #[must_use]
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Return close value
    #[must_use]
    pub fn close(&self) -> f64 {
        self.close
    }
}

/// Struct to store Forex data after forex API call
#[derive(Debug, Default)]
pub struct Forex {
    meta_data: MetaData,
    data: Vec<Data>,
}

impl Forex {
    /// Return information of data
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let information = forex.information();
    ///     assert_eq!(information, "FX Intraday (5min) Time Series");
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return from symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let symbol_from = forex.symbol_from();
    ///     assert_eq!(symbol_from, "EUR");
    /// }
    /// ```
    #[must_use]
    pub fn symbol_from(&self) -> &str {
        self.return_meta_string("from symbol")
    }

    /// Return to symbol
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let symbol_to = forex.symbol_to();
    ///     assert_eq!(symbol_to, "USD");
    /// }
    /// ```
    #[must_use]
    pub fn symbol_to(&self) -> &str {
        self.return_meta_string("to symbol")
    }

    /// Return last refreshed time
    #[must_use]
    pub fn last_refreshed(&self) -> &str {
        self.return_meta_string("last refreshed")
    }

    /// Return time zone of all data time
    #[must_use]
    pub fn time_zone(&self) -> &str {
        self.return_meta_string("time zone")
    }

    /// Return interval for intraday
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let interval = forex.interval();
    ///     assert_eq!(interval.unwrap(), "5min");
    /// }
    /// ```
    #[must_use]
    pub fn interval(&self) -> Option<&str> {
        self.operate_option_meta_value("interval")
    }

    /// Return output size which can be full or compact
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(alpha_vantage::forex::ForexFunction::IntraDay, "EUR", "USD")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let output_size = forex.output_size();
    ///     assert_eq!(output_size.unwrap(), "Full size");
    /// }
    /// ```
    #[must_use]
    pub fn output_size(&self) -> Option<&str> {
        self.operate_option_meta_value("output size")
    }

    /// Method return Data
    #[must_use]
    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    /// Return a meta data field
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "from symbol" => &self.meta_data.from_symbol,
            "to symbol" => &self.meta_data.to_symbol,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }

    /// Convert Option meta data field as a Option<&str>
    fn operate_option_meta_value(&self, which_val: &str) -> Option<&str> {
        let value = match which_val {
            "interval" => &self.meta_data.interval,
            "output size" => &self.meta_data.output_size,
            _ => &None,
        };
        value.as_deref()
    }
}

/// Data Helper
#[derive(Clone, Debug, Deserialize)]
struct DataHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
}

/// struct which helps for collecting forex data from website
#[derive(Debug, Deserialize)]
pub(crate) struct ForexHelper {
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    forex: Option<HashMap<String, HashMap<String, DataHelper>>>,
}

impl ForexHelper {
    /// convert `ForexHelper` to `Forex`
    fn convert(self) -> Result<Forex> {
        if self.meta_data.is_none() || self.forex.is_none() {
            return Err(Error::EmptyResponse);
        }

        let meta_data = self.meta_data.unwrap();

        let information = &meta_data["1. Information"];
        let from_symbol = &meta_data["2. From Symbol"];
        let to_symbol = &meta_data["3. To Symbol"];

        let mut last_refreshed = meta_data.get("4. Last Refreshed");
        if last_refreshed.is_none() {
            last_refreshed = meta_data.get("5. Last Refreshed");
        };

        let time_zone_value = meta_data.get("5. Time Zone").unwrap_or_else(|| {
            meta_data.get("6. Time Zone").unwrap_or_else(|| {
                meta_data
                    .get("7. Time Zone")
                    .expect("time zone contains None value")
            })
        });

        let mut output_size_value = meta_data.get("4. Output Size");
        if output_size_value.is_none() {
            output_size_value = meta_data.get("6. Output Size");
        }

        let interval = meta_data.get("5. Interval");

        let meta_data = MetaData {
            information: information.to_string(),
            from_symbol: from_symbol.to_string(),
            to_symbol: to_symbol.to_string(),
            last_refreshed: last_refreshed
                .expect("last refreshed value contains None")
                .to_string(),
            interval: interval.map(ToString::to_string),
            output_size: output_size_value.map(ToString::to_string),
            time_zone: time_zone_value.to_string(),
        };
        let mut data_entries: Vec<Data> = Vec::new();
        for hash in self.forex.unwrap().values() {
            for val in hash.keys() {
                let data_helper = hash
                    .get(val)
                    .expect("failed to get value from Forex hashmap");

                data_entries.push(Data {
                    time: val.to_string(),
                    open: data_helper.open,
                    high: data_helper.high,
                    low: data_helper.low,
                    close: data_helper.close,
                });
            }
        }

        Ok(Forex {
            data: data_entries,
            meta_data,
        })
    }
}

impl FindData for Vec<Data> {
    #[must_use]
    fn find(&self, time: &str) -> Option<&<Self as IntoIterator>::Item> {
        self.iter().find(|&data| data.time == time)
    }

    #[must_use]
    fn latest(&self) -> <Self as IntoIterator>::Item {
        let mut latest = &Data::default();
        for data in self {
            if latest.time < data.time {
                latest = data;
            }
        }
        latest.clone()
    }

    fn latest_n(&self, n: usize) -> Result<Vec<&<Self as IntoIterator>::Item>> {
        let mut time_list = self.iter().map(|data| &data.time).collect::<Vec<_>>();
        time_list.sort_by_key(|w| cmp::Reverse(*w));

        if n > time_list.len() {
            return Err(Error::DesiredNumberOfDataNotPresent(time_list.len()));
        }

        let mut full_list = Vec::<&Data>::new();

        for time in &time_list[0..n] {
            full_list.push(self.find(time).unwrap());
        }

        Ok(full_list)
    }
}

/// Builder to create `Forex`
pub struct ForexBuilder<'a> {
    api_client: &'a ApiClient,
    function: ForexFunction,
    from_symbol: &'a str,
    to_symbol: &'a str,
    interval: Option<TimeSeriesInterval>,
    output_size: Option<OutputSize>,
}

impl<'a> ForexBuilder<'a> {
    crate::json_data_struct!(Forex, ForexHelper);

    /// Create new `ForexBuilder` from `APIClient`
    #[must_use]
    pub fn new(
        api_client: &'a ApiClient,
        function: ForexFunction,
        from_symbol: &'a str,
        to_symbol: &'a str,
    ) -> Self {
        Self {
            api_client,
            function,
            from_symbol,
            to_symbol,
            interval: None,
            output_size: None,
        }
    }

    /// Define time series interval for forex
    #[must_use]
    pub fn interval(mut self, interval: TimeSeriesInterval) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Define output size for intraday or daily forex
    #[must_use]
    pub fn output_size(mut self, output_size: OutputSize) -> Self {
        self.output_size = Some(output_size);
        self
    }

    fn create_url(&self) -> String {
        let function = match self.function {
            ForexFunction::IntraDay => "FX_INTRADAY",
            ForexFunction::Daily => "FX_DAILY",
            ForexFunction::Weekly => "FX_WEEKLY",
            ForexFunction::Monthly => "FX_MONTHLY",
        };

        let mut url = format!(
            "query?function={}&from_symbol={}&to_symbol={}",
            function, self.from_symbol, self.to_symbol
        );

        if let Some(forex_interval) = &self.interval {
            let interval = match forex_interval {
                TimeSeriesInterval::OneMin => "1min",
                TimeSeriesInterval::FiveMin => "5min",
                TimeSeriesInterval::FifteenMin => "15min",
                TimeSeriesInterval::ThirtyMin => "30min",
                TimeSeriesInterval::SixtyMin => "60min",
            };
            url.push_str(&format!("&interval={interval}"));
        };

        if let Some(forex_output_size) = &self.output_size {
            let size = match forex_output_size {
                OutputSize::Full => "full",
                OutputSize::Compact => "compact",
            };
            url.push_str(&format!("&outputsize={size}"));
        }

        url
    }
}

/// Enum for declaring function for forex function by defining which type of
/// forex series to be returned
#[derive(Clone)]
pub enum ForexFunction {
    /// returns intraday time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    IntraDay,
    /// returns the daily time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    Daily,
    /// returns the weekly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime.
    Weekly,
    /// returns the monthly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime
    Monthly,
}
