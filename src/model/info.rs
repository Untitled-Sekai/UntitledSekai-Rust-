use serde::{Serialize, Deserialize};
use crate::model::form::ServerForm;
use crate::model::srl::Srl;
use crate::model::sections::ServerItemSectionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeItemInfo {
    pub creates: Option<Vec<ServerForm>>,
    pub searches: Option<Vec<ServerForm>>,
    pub sections: Option<Vec<ServerItemSectionType>>,
    pub banner: Option<Srl>
}