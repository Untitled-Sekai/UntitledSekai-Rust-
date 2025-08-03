use serde::{Serialize, Deserialize};
use crate::model::options::servermultioption::MultiOptionValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSelectOption {
    pub query: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    #[serde(rename = "type")]
    pub option_type: String,
    pub def: String,
    pub values: Vec<MultiOptionValue>,
}