use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUserProfile {
    pub id: String,
    pub handle: String,
    pub name: String,
    #[serde(rename = "avatarType")]
    pub avatartype: String,
    #[serde(rename = "avatarForegroundType")]
    pub avatarforegroundtype: String,
    #[serde(rename = "avatarForegroundColor")]
    pub avatarforegroundcolor: String,
    #[serde(rename = "avatarBackgroundType")]
    pub avatarbackgroundtype: String,
    #[serde(rename = "avatarBackgroundColor")]
    pub avatarbackgroundcolor: String,
    #[serde(rename = "bannerType")]
    pub bannertype: String,
    #[serde(rename = "aboutMe")]
    pub aboutme: String,
    pub favorites: Vec<String>,
}

// リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAuthenticateRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub address: String,
    pub time: u64,
    #[serde(rename = "userProfile")]
    pub user_profile: ServiceUserProfile,
}

// レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAuthenticateResponse {
    pub session: String,
    pub expiration: u64,
}