mod contract;
mod functions;
mod intervals;
mod option_type;
mod symbol;

pub use contract::Contract;
pub use functions::Function;
pub use intervals::TimeInterval;
pub use option_type::OptionType;
pub use symbol::Symbol;

use serde::Deserialize;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
pub struct TimeSeriesDaily {
    #[serde(rename = "Meta Data")]
    metadata: HashSet<String>,
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<String, String>,
}
