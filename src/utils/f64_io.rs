use serde::{self, de, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let float_str = String::deserialize(deserializer)?;
    float_str.parse::<f64>().map_err(|_err| {
        de::Error::invalid_value(
            de::Unexpected::Str(float_str.as_str()),
            &"a string representation of an f64",
        )
    })
}
