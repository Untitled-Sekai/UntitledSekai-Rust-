use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSliderOption {
    pub query: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    #[serde(rename = "type")]
    pub option_type: String,
    pub def: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub unit: Option<String>,
}