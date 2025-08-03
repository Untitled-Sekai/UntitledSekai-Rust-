// サーバーのinfoを提供するハンドラー

use axum::response::Json;
use crate::model::serverinfo::{ServerInfo, Button, ButtonType};
use crate::model::srl::Srl;
use serde_json::json;

pub async fn info_handler() -> Json<ServerInfo> {
    let title = "Untitled Sekai".to_string();
    let description = "Rust製のUntitled Sekaiサーバー".to_string();
    let buttons = vec![
        Button { button_type: ButtonType::Authentication },
        Button { button_type: ButtonType::Level },
        Button { button_type: ButtonType::Skin },
        Button { button_type: ButtonType::Background },
        Button { button_type: ButtonType::Effect },
        Button { button_type: ButtonType::Particle },
        Button { button_type: ButtonType::Engine },
        Button { button_type: ButtonType::Configuration },
    ];

    let banner = Srl {
        hash: Some("banner".to_string()),
        url: Some("/assets/banner".to_string()),
    };

    let server_info = ServerInfo {
        title,
        description: Some(description),
        buttons,
        configuration: Some(json!({ "options": [] })),
        banner,
    };

    Json(server_info)
}