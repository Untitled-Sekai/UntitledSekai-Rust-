use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags {
    pub title: String,
    pub icon: String,
}