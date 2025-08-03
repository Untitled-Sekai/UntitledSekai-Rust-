use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Srl {
    pub hash: Option<String>,
    pub url: Option<String>,
}