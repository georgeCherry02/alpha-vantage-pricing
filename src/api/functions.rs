use std::string::ToString;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Function {
    #[serde(rename = "HISTORICAL_OPTIONS")]
    HistoricalOptions,
    #[serde(rename = "TIME_SERIES_DAILY")]
    TimeSeriesDaily,
}

impl ToString for Function {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .map(|s| -> String {
                let mut c = s.chars();
                c.next();
                c.next_back();
                String::from(c.as_str())
            })
            .unwrap_or_default()
    }
}
