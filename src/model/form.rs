use serde::{Serialize, Deserialize};

use crate::model::options::servertextoption::ServerTextOption;
use crate::model::options::servertextareaoption::ServerTextAreaOption;
use crate::model::options::serverslideroption::ServerSliderOption;
use crate::model::options::servermultioption::ServerMultiOption;
use crate::model::options::serverselectoption::ServerSelectOption;
use crate::model::options::servertoggleoption::ServerToggleOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerOption {
    #[serde(rename = "text")]
    Text(ServerTextOption),
    #[serde(rename = "textarea")]
    TextArea(ServerTextAreaOption),
    #[serde(rename = "slider")]
    Slider(ServerSliderOption),
    #[serde(rename = "toggle")]
    Toggle(ServerToggleOption),
    #[serde(rename = "select")]
    Select(ServerSelectOption),
    #[serde(rename = "multi")]
    Multi(ServerMultiOption),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerForm {
    #[serde(rename = "type")]
    pub option_type: String,
    pub title: String,
    pub description: Option<String>,
    pub help: Option<String>,
    pub requireconfirmation: bool,
    pub options: Vec<ServerOption>,
}