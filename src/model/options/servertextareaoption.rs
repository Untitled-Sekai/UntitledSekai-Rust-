use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTextAreaOption {
    pub query: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    #[serde(rename = "type")]
    pub option_type: String,
    pub def: String,
    pub placeholder: String,
    pub limit: f32,
    pub shortcuts: Vec<String>,
}