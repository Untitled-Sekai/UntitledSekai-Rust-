use serde::{Deserialize, Serialize};

use crate::model::form::ServerForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerItemSectionType {
    pub title: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub itemType: String,
    pub items: Vec<String>,
    pub search: Option<ServerForm>,
    pub searchValue: Option<String>,
}