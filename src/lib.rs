mod api;
mod utils;
mod result;

use api::Symbol;
use result::VantageResult;

use chrono::NaiveDate;

fn get_historical_options(symbol: Symbol, date: NaiveDate) -> VantageResult<()> {
    Ok(())
}
