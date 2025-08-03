// LevelのuseItemの定義

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UseItem<T> {
    Default {
        #[serde(rename = "useDefault")]
        usedefault: bool,
    },
    Item {
        item: T,
    }
}

impl<T> UseItem<T>{
    pub fn default() -> Self {
        UseItem::Default { usedefault: true }
    }

    pub fn item(item: T) -> Self {
        UseItem::Item { item }
    }

    pub fn is_default(&self) -> bool {
        matches!(self, UseItem::Default { usedefault: true })
    }

    pub fn get_item(&self) -> Option<&T> {
        match self {
            UseItem::Item { item } => Some(item),
            _ => None,
        }
    }
}