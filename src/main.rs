mod api;
mod networking;
mod result;
mod utils;

use api::{Contract, Function, Symbol};
use networking::{make_query, query_element, ToUrl};
use result::VantageResult;

use chrono::NaiveDate;
use log::info;

fn get_arbitrary_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2024, 8, 14).unwrap()
}

async fn view_ibm_options() -> VantageResult<()> {
    let function = query_element("function", Function::HistoricalOptions);
    let symbol = query_element("symbol", Symbol::new("IBM"));
    let date = query_element("date", get_arbitrary_date());
    let query: Vec<&dyn ToUrl> = vec![&function, &symbol, &date];
    let contract: Vec<Contract> = make_query(query).await?;
    info!("{:?}", contract);
    info!("Fetched {} contracts", contract.len());
    Ok(())
}

async fn view_ibm_timeseries() -> VantageResult<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> VantageResult<()> {
    env_logger::init();
    view_ibm_options().await
}
