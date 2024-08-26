//! Module for Economic Indicator
//!
//! APIs under this section provide key US economic indicators frequently used
//! for investment strategy formulation and application development.
//!
//! You can read about [Economic Indicator][economic_indicator] API and what
//! it returns on alphavantage documentation
//!
//! [economic_indicator]: https://www.alphavantage.co/documentation/#economic-indicators

use std::cmp;

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::from_str;
use crate::error::{Error, Result};
use crate::vec_trait::FindData;

/// Struct for storing a data values
#[derive(Default, Debug, Deserialize, Clone)]
pub struct Data {
    date: String,
    #[serde(deserialize_with = "from_str")]
    value: f64,
}

impl Data {
    /// Return date
    #[must_use]
    pub fn date(&self) -> &str {
        &self.date
    }

    /// Return value for Data
    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl FindData for Vec<Data> {
    #[must_use]
    fn find(&self, time: &str) -> Option<&<Self as IntoIterator>::Item> {
        self.iter().find(|&data| data.date == time)
    }

    #[must_use]
    fn latest(&self) -> <Self as IntoIterator>::Item {
        let mut latest = &Data::default();
        for data in self {
            if latest.date < data.date {
                latest = data;
            }
        }
        latest.clone()
    }

    fn latest_n(&self, n: usize) -> Result<Vec<&<Self as IntoIterator>::Item>> {
        let mut date_list = self.iter().map(|data| &data.date).collect::<Vec<_>>();
        date_list.sort_by_key(|w| cmp::Reverse(*w));

        if n > date_list.len() {
            return Err(Error::DesiredNumberOfDataNotPresent(date_list.len()));
        }

        let mut full_list = Vec::<&Data>::new();

        for date in &date_list[0..n] {
            full_list.push(self.find(date).unwrap());
        }

        Ok(full_list)
    }
}

/// Struct for indicator
#[derive(Default, Debug)]
pub struct EconomicIndicator {
    name: String,
    interval: String,
    unit: String,
    data: Vec<Data>,
}

impl EconomicIndicator {
    /// Return name of economic indicator
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Return interval of economic indicator
    #[must_use]
    pub fn interval(&self) -> &String {
        &self.interval
    }

    /// Return unit of economic indicator
    #[must_use]
    pub fn unit(&self) -> &String {
        &self.unit
    }

    /// Return data as a vector
    #[must_use]
    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }
}

/// Struct for helping indicator struct
#[derive(Deserialize)]
pub(crate) struct EconomicIndicatorHelper {
    name: Option<String>,
    interval: Option<String>,
    unit: Option<String>,
    data: Option<Vec<Data>>,
}

impl EconomicIndicatorHelper {
    fn convert(self) -> Result<EconomicIndicator> {
        if self.name.is_none()
            || self.interval.is_none()
            || self.unit.is_none()
            || self.data.is_none()
        {
            return Err(Error::EmptyResponse);
        }
        Ok(EconomicIndicator {
            name: self.name.unwrap(),
            interval: self.interval.unwrap(),
            unit: self.unit.unwrap(),
            data: self.data.unwrap(),
        })
    }
}

/// Builder to help create `EconomicIndicator`
pub struct EconomicIndicatorBuilder<'a> {
    api_client: &'a ApiClient,
    function: &'a str,
    interval: Option<EconomicIndicatorInterval>,
    maturity: Option<EconomicIndicatorMaturity>,
}

impl<'a> EconomicIndicatorBuilder<'a> {
    crate::json_data_struct!(EconomicIndicator, EconomicIndicatorHelper);

    /// Create new `EconomicIndicatorBuilder` form `APIClient`
    #[must_use]
    pub fn new(api_client: &'a ApiClient, function: &'a str) -> Self {
        Self {
            api_client,
            function,
            interval: None,
            maturity: None,
        }
    }

    /// Set interval for API
    pub fn interval(&mut self, interval: EconomicIndicatorInterval) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    /// Set maturity for API
    pub fn maturity(&mut self, maturity: EconomicIndicatorMaturity) -> &mut Self {
        self.maturity = Some(maturity);
        self
    }

    fn create_url(&self) -> String {
        let mut created_link = format!("query?function={}", &self.function);

        if let Some(interval) = &self.interval {
            match interval {
                EconomicIndicatorInterval::Daily => created_link.push_str("&interval=daily"),
                EconomicIndicatorInterval::Weekly => created_link.push_str("&interval=weekly"),
                EconomicIndicatorInterval::Monthly => created_link.push_str("&interval=monthly"),
                EconomicIndicatorInterval::Quarterly => {
                    created_link.push_str("&interval=quarterly");
                }
                EconomicIndicatorInterval::Annually => created_link.push_str("&interval=annually"),
            }
        }

        if let Some(maturity) = &self.maturity {
            match maturity {
                EconomicIndicatorMaturity::ThreeMonth => created_link.push_str("&maturity=3month"),
                EconomicIndicatorMaturity::FiveYear => created_link.push_str("&maturity=5year"),
                EconomicIndicatorMaturity::TenYear => created_link.push_str("&maturity=10year"),
                EconomicIndicatorMaturity::ThirtyYear => created_link.push_str("&maturity=30year"),
            }
        }

        created_link
    }
}

/// Enum for declaring interval for economic indicator
#[derive(Clone)]
pub enum EconomicIndicatorInterval {
    /// daily interval
    Daily,
    /// weekly interval
    Weekly,
    /// monthly interval
    Monthly,
    /// quarterly interval
    Quarterly,
    /// annually interval
    Annually,
}

/// Enum for declaring maturity for economic indicator
#[derive(Clone)]
pub enum EconomicIndicatorMaturity {
    /// 3 month maturity
    ThreeMonth,
    /// 5 year maturity
    FiveYear,
    /// 10 year maturity
    TenYear,
    /// 30 year maturity
    ThirtyYear,
}
