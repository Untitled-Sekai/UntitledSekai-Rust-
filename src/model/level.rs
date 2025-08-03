use serde::{Serialize, Deserialize};
use crate::model::tags::Tags;
use crate::model::srl::Srl;
use crate::model::useitem::UseItem;

use crate::model::engine::EngineItem;
use crate::model::skin::SkinItem;
use crate::model::background::BackgroundItem;
use crate::model::effect::EffectItem;
use crate::model::particle::ParticleItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Levelitem {
    // metadata
    pub name: String,
    pub source: String,
    pub version: f32,
    pub rating: f32,
    pub title: String,
    pub artists: String,
    pub author: String,
    pub tags: Option<Tags>,
    
    // engine
    pub engine: EngineItem,
    
    // items
    #[serde(rename = "useItem")]
    pub useSkin: UseItem<SkinItem>,
    #[serde(rename = "useItem")]
    pub useBackground: UseItem<BackgroundItem>,
    #[serde(rename = "useItem")]
    pub useEffect: UseItem<EffectItem>,
    #[serde(rename = "useItem")]
    pub useParticle: UseItem<ParticleItem>,

    // content
    pub cover: Srl,
    pub bgm: Srl,
    pub preview: Option<Srl>,
    pub data: Srl,
}