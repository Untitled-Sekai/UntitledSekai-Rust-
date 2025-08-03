use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiOptionValue {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMultiOption {
    pub query: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    #[serde(rename = "type")]
    pub option_type: String,
    pub def: Vec<bool>,
    pub values: Vec<MultiOptionValue>,
}

impl ServerMultiOption {
    pub fn new(
        query: String,
        name: String,
        description: Option<String>,
        required: bool,
        def: Vec<bool>,
        values: Vec<MultiOptionValue>,
    ) -> Self {
        Self {
            query,
            name,
            description,
            required,
            option_type: "multi".to_string(),
            def,
            values,
        }
    }
}