use serde::Deserialize;

use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Symbol {
    symbol: String,
}

impl Symbol {
    pub fn new(symbol: &str) -> Self {
        Symbol {
            symbol: String::from(symbol),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
