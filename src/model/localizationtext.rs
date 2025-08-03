use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationText {
    pub en: Option<String>,
    pub ja: Option<String>,
}