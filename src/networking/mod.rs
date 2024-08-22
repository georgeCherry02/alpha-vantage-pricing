mod query;
mod utils;

use query::{construct_url, Query};
pub use query::{query_element, ToUrl};
use utils::get_api_key;

use crate::result::VantageResult;

use serde::{de::DeserializeOwned, Deserialize};

use log::debug;

use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct VantageResponse<DataType> {
    endpoint: String,
    message: String,
    data: Vec<DataType>,
}

fn parse_response<T: Debug>(response: VantageResponse<T>) -> VantageResult<Vec<T>> {
    match response.message.as_str() {
        "success" => Ok(response.data),
        _ => Err(response.message.into()),
    }
}

async fn get_request<'a, ResponseDataType: Debug + DeserializeOwned>(
    query: Query<'a>,
) -> VantageResult<Vec<ResponseDataType>> {
    reqwest::Client::new()
        .get(construct_url(query))
        .send()
        .await?
        .json::<VantageResponse<ResponseDataType>>()
        .await
        .map(parse_response)?
}

pub async fn make_query<'a, ResponseDataType: Debug + DeserializeOwned>(
    mut query: Query<'a>,
) -> VantageResult<Vec<ResponseDataType>> {
    let apikey = query_element("apikey", get_api_key()?);
    query.push(&apikey);
    debug!("Making query: {:?}", query);
    get_request(query).await
}
