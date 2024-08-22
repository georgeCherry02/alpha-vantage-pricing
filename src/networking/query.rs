use super::utils::get_url;

use log::debug;

use std::fmt::Debug;
use std::string::ToString;

#[derive(Debug)]
pub struct TypedQueryElement<T>
where
    T: ToString,
{
    name: String,
    value: T,
}

pub fn query_element<T: ToString>(name: &str, value: T) -> TypedQueryElement<T> {
    TypedQueryElement {
        name: String::from(name),
        value,
    }
}

pub trait ToUrl: Debug {
    fn to_url(&self) -> String;
}

impl<T: Debug + ToString> ToUrl for TypedQueryElement<T> {
    fn to_url(&self) -> String {
        format!("{}={}", self.name, self.value.to_string())
    }
}

pub type Query<'a> = Vec<&'a dyn ToUrl>;

pub fn construct_url(query: Query) -> String {
    let url_query_segment = query
        .into_iter()
        .map(|q| q.to_url())
        .collect::<Vec<String>>()
        .join("&");
    let url = format!("{}?{}", get_url(), url_query_segment);
    debug!("Construted url: {}", url);
    url
}
