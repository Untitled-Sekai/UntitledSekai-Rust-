use serde::{Serialize, Deserialize};
use crate::model::tags::Tags;
use crate::model::srl::Srl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinItem {
    name: String,
    source: String,
    version: f32,

    title: String,
    subtitle: String,
    author: String,

    tags: Option<Tags>,

    thumbnail: Srl,
    data: Srl,
    texture: Srl,
}