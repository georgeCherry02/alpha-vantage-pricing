use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum OptionType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "put")]
    Put,
}
