use std::env;

use crate::result::{VantageError, VantageResult};

const ALPHA_VANTAGE_ENDPOINT: &'static str = "https://www.alphavantage.co/query";

pub fn get_url() -> String {
    String::from(ALPHA_VANTAGE_ENDPOINT)
}

pub fn get_api_key() -> VantageResult<String> {
    env::var("ALPHA_VANTAGE_API_KEY").map_err(|_| VantageError::new("Failed to find API key"))
}
