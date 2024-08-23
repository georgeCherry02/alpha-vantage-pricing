mod query;
mod utils;

use query::{construct_url, Query};
pub use query::{query_element, ToUrl};
use utils::get_api_key;

use crate::result::VantageResult;

use log::{debug, error};
use serde::{de::DeserializeOwned, Deserialize};

use std::fmt::Debug;

// Turns out this format is only used in the options data response...
#[derive(Debug, Deserialize)]
pub struct VantageResponse<DataType> {
    endpoint: String,
    message: String,
    data: Vec<DataType>,
}

fn handle_response<T: Debug>(response: VantageResponse<T>) -> VantageResult<Vec<T>> {
    match response.message.as_str() {
        "success" => {
            debug!("Received response: {:?}", response.data);
            Ok(response.data)
        }
        _ => {
            error!("Received non-success message: {}", response.message);
            Err(response.message.into())
        }
    }
}

async fn dispatch_get_request<'a>(mut query: Query<'a>) -> VantageResult<reqwest::Response> {
    let apikey = query_element("apikey", get_api_key()?);
    query.push(&apikey);
    Ok(reqwest::Client::new()
        .get(construct_url(query))
        .send()
        .await?)
}

pub async fn make_query<'a, ResponseDataType: Debug + DeserializeOwned>(
    query: Query<'a>,
) -> VantageResult<Vec<ResponseDataType>> {
    dispatch_get_request(query)
        .await?
        .json::<VantageResponse<ResponseDataType>>()
        .await
        .map(handle_response)?
}

pub async fn make_query_direct<'a, ResponseType: Debug + DeserializeOwned>(
    query: Query<'a>,
) -> VantageResult<ResponseType> {
    Ok(dispatch_get_request(query)
        .await?
        .json::<ResponseType>()
        .await
        .map(|r| {
            debug!("Received response: {:?}", r);
            r
        })?)
}
