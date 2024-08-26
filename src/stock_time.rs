//! Module for stock time series
//!
//! This suite of APIs provide realtime and historical global equity data in 4
//! different temporal resolutions: (1) daily, (2) weekly, (3) monthly, and (4)
//! intraday. Daily, weekly, and monthly time series contain 20+ years of
//! historical data
//!
//! You can read about [Stock Time][stock_time] API and what it returns
//! on alphavantage documentation
//!
//! [stock_time]: https://www.alphavantage.co/documentation/#time-series-data

use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

use serde::Deserialize;

use crate::api::{ApiClient, OutputSize, TimeSeriesInterval};
use crate::deserialize::from_str;
use crate::error::{Error, Result};
use crate::vec_trait::FindData;

/// Struct for storing Meta Data value
#[derive(Debug, Clone, Default)]
pub struct MetaData {
    information: String,
    symbol: String,
    last_refreshed: String,
    interval: Option<String>,
    output_size: Option<String>,
    time_zone: String,
}

/// Struct for Data value
#[derive(Default, Debug, Clone)]
pub struct Data {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adjusted_close: Option<f64>,
    volume: u64,
    dividend_amount: Option<f64>,
    split_coefficient: Option<f64>,
}

impl Data {
    /// Get time
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return open
    #[must_use]
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Return high
    #[must_use]
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Return low
    #[must_use]
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Return close
    #[must_use]
    pub fn close(&self) -> f64 {
        self.close
    }

    /// Return adjusted
    #[must_use]
    pub fn adjusted(&self) -> Option<f64> {
        self.adjusted_close
    }

    /// Return volume
    #[must_use]
    pub fn volume(&self) -> u64 {
        self.volume
    }

    /// Return dividend
    #[must_use]
    pub fn dividend(&self) -> Option<f64> {
        self.dividend_amount
    }

    /// Return split dividend
    #[must_use]
    pub fn split(&self) -> Option<f64> {
        self.split_coefficient
    }
}

/// Struct for storing time series data
#[derive(Debug, Default)]
pub struct TimeSeries {
    meta_data: MetaData,
    data: Vec<Data>,
}

impl TimeSeries {
    /// Return information present in meta data
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock_time = api
    ///         .stock_time(alpha_vantage::stock_time::StockFunction::IntraDay, "MSFT")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let information = stock_time.information();
    ///     assert_eq!(
    ///         information,
    ///         "Intraday (5min) open, high, low, close prices and volume"
    ///     );
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return symbol for which time series function is called
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock_time = api
    ///         .stock_time(alpha_vantage::stock_time::StockFunction::IntraDay, "MSFT")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let symbol = stock_time.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.return_meta_string("symbol")
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

    /// Time series interval between two consecutive data
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock_time = api
    ///         .stock_time(alpha_vantage::stock_time::StockFunction::IntraDay, "MSFT")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let interval = stock_time.interval();
    ///     assert_eq!(interval.unwrap(), "5min");
    /// }
    /// ```
    #[must_use]
    pub fn interval(&self) -> Option<&str> {
        self.operate_option_meta_value("interval")
    }

    /// Output Size of intraday which can be either Full or compact
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock_time = api
    ///         .stock_time(alpha_vantage::stock_time::StockFunction::IntraDay, "MSFT")
    ///         .interval(alpha_vantage::api::TimeSeriesInterval::FiveMin)
    ///         .output_size(alpha_vantage::api::OutputSize::Full)
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let output_size = stock_time.output_size();
    ///     assert_eq!(output_size.unwrap(), "Full size");
    /// }
    /// ```
    #[must_use]
    pub fn output_size(&self) -> Option<&str> {
        self.operate_option_meta_value("output size")
    }

    /// Return Data
    #[must_use]
    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    /// Return a meta data value as a form of String
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "symbol" => &self.meta_data.symbol,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }

    /// Return Option metadata value as a Result form of String
    fn operate_option_meta_value(&self, which_val: &str) -> Option<&str> {
        let value = match which_val {
            "interval" => &self.meta_data.interval,
            "output size" => &self.meta_data.output_size,
            _ => &None,
        };
        value.as_deref()
    }
}

/// Helper struct to store non adjusted data
#[derive(Clone, Deserialize)]
struct DataHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
    #[serde(rename = "5. volume", deserialize_with = "from_str")]
    volume: u64,
}

/// Helper struct to store adjusted data
#[derive(Deserialize, Clone)]
struct AdjustedHelper {
    #[serde(rename = "1. open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "from_str")]
    close: f64,
    #[serde(rename = "5. adjusted close")]
    adjusted_close: Option<String>,
    #[serde(rename = "6. volume", deserialize_with = "from_str")]
    volume: u64,
    #[serde(rename = "7. dividend amount")]
    dividend_amount: Option<String>,
    #[serde(rename = "8. split coefficient")]
    split_coefficient: Option<String>,
}

/// helper struct for `TimeSeries` which deserialize JSON
#[derive(Deserialize)]
pub(crate) struct TimeSeriesHelper {
    #[serde(rename = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten)]
    time_series: Option<HashMap<String, HashMap<String, DataHelper>>>,
    #[serde(flatten)]
    adjusted_series: Option<HashMap<String, HashMap<String, AdjustedHelper>>>,
}

impl TimeSeriesHelper {
    /// Convert `TimeSeriesHelper` to `TimeSeries`
    fn convert(self) -> Result<TimeSeries> {
        if self.meta_data.is_none()
            || (self.time_series.is_none() && self.adjusted_series.is_none())
        {
            return Err(Error::EmptyResponse);
        }

        let meta_data = self.meta_data.unwrap();
        let information = &meta_data["1. Information"];
        let symbol = &meta_data["2. Symbol"];
        let last_refreshed = &meta_data["3. Last Refreshed"];
        let interval = meta_data.get("4. Interval");

        let mut output_size = meta_data.get("4. Output Size");
        if output_size.is_none() {
            output_size = meta_data.get("5. Output Size");
        }

        let time_zone = meta_data.get("4. Time Zone").unwrap_or_else(|| {
            meta_data.get("5. Time Zone").unwrap_or_else(|| {
                meta_data
                    .get("6. Time Zone")
                    .expect("time zone value is None")
            })
        });

        let meta_data = MetaData {
            information: information.to_string(),
            symbol: symbol.to_string(),
            last_refreshed: last_refreshed.to_string(),
            interval: interval.map(ToString::to_string),
            output_size: output_size.map(ToString::to_string),
            time_zone: time_zone.to_string(),
        };

        let mut data_value: Vec<Data> = Vec::new();

        if let Some(time_series) = self.time_series {
            for hash in time_series.values() {
                for val in hash.keys() {
                    let data_helper = hash
                        .get(val)
                        .expect("failed to get value from stock time hashmap");

                    data_value.push(Data {
                        time: val.to_string(),
                        open: data_helper.open,
                        high: data_helper.high,
                        low: data_helper.low,
                        close: data_helper.close,
                        volume: data_helper.volume,
                        ..Data::default()
                    });
                }
            }
        }

        if let Some(adjusted_series) = self.adjusted_series {
            for hash in adjusted_series.values() {
                for val in hash.keys() {
                    let data_helper = hash
                        .get(val)
                        .expect("failed to get value from adjusted series");

                    data_value.push(Data {
                        time: val.to_string(),
                        open: data_helper.open,
                        high: data_helper.high,
                        low: data_helper.low,
                        close: data_helper.close,
                        volume: data_helper.volume,
                        adjusted_close: option_from_str(&data_helper.adjusted_close),
                        split_coefficient: option_from_str(&data_helper.split_coefficient),
                        dividend_amount: option_from_str(&data_helper.dividend_amount),
                    });
                }
            }
        }

        Ok(TimeSeries {
            data: data_value,
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

// convert string to optional T
fn option_from_str<T>(val: &Option<String>) -> Option<T>
where
    T: FromStr,
    T::Err: std::error::Error,
{
    val.as_ref().map(|s| T::from_str(s).unwrap())
}

/// Builder to create new `TimeSeries`
pub struct TimeSeriesBuilder<'a> {
    api_client: &'a ApiClient,
    function: StockFunction,
    symbol: &'a str,
    interval: Option<TimeSeriesInterval>,
    output_size: Option<OutputSize>,
    adjusted: Option<bool>,
}

impl<'a> TimeSeriesBuilder<'a> {
    crate::json_data_struct!(TimeSeries, TimeSeriesHelper);

    /// Create new `TimeSeriesBuilder` form `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, function: StockFunction, symbol: &'a str) -> Self {
        Self {
            api_client,
            function,
            symbol,
            interval: None,
            output_size: None,
            adjusted: None,
        }
    }

    /// Define time series interval for intraday stock time series
    #[must_use]
    pub fn interval(mut self, interval: TimeSeriesInterval) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Define output size for intraday or daily stock time series
    #[must_use]
    pub fn output_size(mut self, output_size: OutputSize) -> Self {
        self.output_size = Some(output_size);
        self
    }

    /// Define if output time series is adjusted by historical split and
    /// dividend events
    #[must_use]
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    fn create_url(&self) -> String {
        let function = match self.function {
            StockFunction::IntraDay => "TIME_SERIES_INTRADAY",
            StockFunction::Daily => "TIME_SERIES_DAILY",
            StockFunction::DailyAdjusted => "TIME_SERIES_DAILY_ADJUSTED",
            StockFunction::Weekly => "TIME_SERIES_WEEKLY",
            StockFunction::WeeklyAdjusted => "TIME_SERIES_WEEKLY_ADJUSTED",
            StockFunction::Monthly => "TIME_SERIES_MONTHLY",
            StockFunction::MonthlyAdjusted => "TIME_SERIES_MONTHLY_ADJUSTED",
        };

        let mut url = format!("query?function={function}&symbol={}", self.symbol);

        if let Some(stock_time_interval) = &self.interval {
            let interval = match stock_time_interval {
                TimeSeriesInterval::OneMin => "1min",
                TimeSeriesInterval::FiveMin => "5min",
                TimeSeriesInterval::FifteenMin => "15min",
                TimeSeriesInterval::ThirtyMin => "30min",
                TimeSeriesInterval::SixtyMin => "60min",
            };
            url.push_str(&format!("&interval={interval}"));
        };

        if let Some(stock_time_output_size) = &self.output_size {
            let size = match stock_time_output_size {
                OutputSize::Full => "full",
                OutputSize::Compact => "compact",
            };
            url.push_str(&format!("&outputsize={size}"));
        }

        if let Some(adjusted) = self.adjusted {
            if adjusted {
                url.push_str("&adjusted=true");
            } else {
                url.push_str("&adjusted=false");
            }
        };

        url
    }
}

/// Enum for declaring function for stock time series by defining which type of
/// series of stock to be returned
#[derive(Clone)]
pub enum StockFunction {
    /// returns intraday time series (timestamp, open, high, low, close, volume)
    /// of the equity specified
    IntraDay,
    /// returns daily time series (date, daily open, daily high, daily low,
    /// daily close, daily volume) of the global equity specified, covering 20+
    /// years of historical data
    Daily,
    /// returns daily time series (date, daily open, daily high, daily low,
    /// daily close, daily volume, daily adjusted close, and split/dividend
    /// events) of the global equity specified, covering 20+ years of historical
    /// data.
    DailyAdjusted,
    /// returns weekly time series (last trading day of each week, weekly open,
    /// weekly high, weekly low, weekly close, weekly volume) of the global
    /// equity specified, covering 20+ years of historical data.
    Weekly,
    /// returns weekly adjusted time series (last trading day of each week,
    /// weekly open, weekly high, weekly low, weekly close, weekly adjusted
    /// close, weekly volume, weekly dividend) of the global equity specified,
    /// covering 20+ years of historical data.
    WeeklyAdjusted,
    /// returns monthly time series (last trading day of each month, monthly
    /// open, monthly high, monthly low, monthly close, monthly volume) of
    /// the global equity specified, covering 20+ years of historical data.
    Monthly,
    /// returns monthly adjusted time series (last trading day of each month,
    /// monthly open, monthly high, monthly low, monthly close, monthly adjusted
    /// close, monthly volume, monthly dividend) of the equity specified,
    /// covering 20+ years of historical data.
    MonthlyAdjusted,
}
