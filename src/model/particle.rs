use serde::{Serialize, Deserialize};
use crate::model::srl::Srl;
use crate::model::tags::Tags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleItem {
    // meta
    pub name: String,
    pub source: String,
    pub version: f32,

    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Option<Tags>,

    // content
    pub thumbnail: Srl,
    pub data: Srl,
    pub texture: Srl,
}