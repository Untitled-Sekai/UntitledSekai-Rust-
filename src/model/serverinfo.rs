use serde::{Serialize, Deserialize};
use crate::model::srl::Srl;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonType {
    Authentication,
    Post,
    Level,
    Skin,
    Background,
    Effect,
    Particle,
    Engine,
    Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    #[serde(rename = "type")]
    pub button_type: ButtonType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub title: String,
    pub description: Option<String>,
    pub buttons: Vec<Button>,
    pub configuration: Option<serde_json::Value>,
    pub banner: Srl,
}