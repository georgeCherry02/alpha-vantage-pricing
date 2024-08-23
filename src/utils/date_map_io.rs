use chrono::NaiveDate;
use serde::{self, de::DeserializeOwned, Deserialize, Deserializer};

use std::collections::HashMap;

pub fn deserialize<'de, D, ValueType: DeserializeOwned>(
    de: D,
) -> Result<HashMap<NaiveDate, ValueType>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HashMap::<String, ValueType>::deserialize(de)?
        .into_iter()
        .flat_map(|(k, v)| NaiveDate::parse_from_str(k.as_str(), "%Y-%m-%d").map(|d| (d, v)))
        .collect())
}
