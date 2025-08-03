use axum::response::Json;
use crate::model::info::ServeItemInfo;

pub async fn levelinfo_handler() -> Json<ServeItemInfo> {
    let creates = Some(vec![]);
    let searches = Some(vec![]);
    let sections = Some(vec![]);
    let banner = None;

    let levelinfo = ServeItemInfo {
        creates,
        searches,
        sections,
        banner
    };

    Json(levelinfo)
}