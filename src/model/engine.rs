use serde::{Serialize, Deserialize};
use crate::model::tags::Tags;
use crate::model::srl::Srl;

use crate::model::skin::SkinItem;
use crate::model::background::BackgroundItem;
use crate::model::effect::EffectItem;
use crate::model::particle::ParticleItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineItem {
    // metadata
    pub name: String,
    pub source: String,
    pub version: f32,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Option<Tags>,

    // path
    pub thumbnail: Srl,
    pub playdata: Srl,
    pub watchdata: Srl,
    pub previewdata: Srl,
    pub tutorialdata: Srl,
    pub rom: Option<Srl>,
    pub configuration: Srl,
    
    // items
    pub skin: SkinItem,
    pub background: BackgroundItem,
    pub effect: EffectItem,
    pub particle: ParticleItem,
}