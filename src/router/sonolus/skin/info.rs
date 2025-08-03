use axum::response::Json;
use crate::model::info::ServeItemInfo;

pub async fn skininfo_handler() -> Json<ServeItemInfo> {
    let creates = Some(vec![]);
    let searches = Some(vec![]);
    let sections = Some(vec![]);
    let banner = None;

    let skininfo = ServeItemInfo {
        creates,
        searches,
        sections,
        banner
    };

    Json(skininfo)
}