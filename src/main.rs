mod api;
mod networking;
mod result;
mod utils;

use api::{Contract, Function, Symbol};
use networking::{make_query, query, ToUrl};
use result::VantageResult;

use chrono::NaiveDate;
use log::info;

fn get_arbitrary_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2024, 8, 14).unwrap()
}

#[tokio::main]
async fn main() -> VantageResult<()> {
    env_logger::init();
    let function = query(String::from("function"), Function::HistoricalOptions);
    let symbol = query(String::from("symbol"), Symbol::new("IBM"));
    let date = query(String::from("date"), get_arbitrary_date());
    let full_query: Vec<&dyn ToUrl> = vec![&function, &symbol, &date];
    let contract: Vec<Contract> = make_query(full_query).await?;
    info!("{:?}", contract);
    info!("Fetched {} contracts", contract.len());
    let strikes = contract
        .into_iter()
        .map(|c| c.strike())
        .collect::<Vec<f64>>();
    let max = strikes.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
    let min = strikes.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
    info!("Highest strike: {}", max.unwrap());
    info!("Lowest strike: {}", min.unwrap());
    Ok(())
}
