use super::OptionType;
use super::Symbol;

use crate::utils::{date_io, f64_io, u32_io};

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contract {
    #[serde(rename = "contractID")]
    contract_id: String,
    #[serde(flatten)]
    symbol: Symbol,
    #[serde(with = "date_io")]
    expiration: NaiveDate,
    #[serde(with = "f64_io")]
    strike: f64,
    #[serde(rename = "type")]
    option_type: OptionType,
    #[serde(with = "f64_io")]
    last: f64,
    #[serde(with = "f64_io")]
    mark: f64,
    #[serde(with = "f64_io")]
    bid: f64,
    #[serde(with = "u32_io")]
    bid_size: u32,
    #[serde(with = "f64_io")]
    ask: f64,
    #[serde(with = "u32_io")]
    ask_size: u32,
    #[serde(with = "u32_io")]
    volume: u32,
    #[serde(with = "u32_io")]
    open_interest: u32,
    #[serde(with = "date_io")]
    date: NaiveDate,
    #[serde(with = "f64_io")]
    implied_volatility: f64,
    #[serde(with = "f64_io")]
    delta: f64,
    #[serde(with = "f64_io")]
    gamma: f64,
    #[serde(with = "f64_io")]
    theta: f64,
    #[serde(with = "f64_io")]
    vega: f64,
    #[serde(with = "f64_io")]
    rho: f64,
}

impl Contract {
    pub fn strike(&self) -> f64 {
        self.strike
    }
}
