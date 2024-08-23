use crate::utils::{date_io, date_map_io, f64_io, u32_io};

use chrono::NaiveDate;
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TimeSeriesData {
    #[serde(rename = "1. open", with = "f64_io")]
    open: f64,
    #[serde(rename = "2. high", with = "f64_io")]
    high: f64,
    #[serde(rename = "3. low", with = "f64_io")]
    low: f64,
    #[serde(rename = "4. close", with = "f64_io")]
    close: f64,
    #[serde(rename = "5. volume", with = "u32_io")]
    volume: u32,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed", with = "date_io")]
    last_refreshed: NaiveDate,
    #[serde(rename = "4. Output Size")]
    output_type: String,
    #[serde(rename = "5. Time Zone")]
    time_zone: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeSeriesDaily {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Time Series (Daily)", with = "date_map_io")]
    time_series: HashMap<NaiveDate, TimeSeriesData>,
}
