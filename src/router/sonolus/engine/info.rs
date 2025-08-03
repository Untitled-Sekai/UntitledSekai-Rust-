use axum::response::Json;
use crate::model::info::ServeItemInfo;

pub async fn engineinfo_handler() -> Json<ServeItemInfo> {
    let creates = Some(vec![]);
    let searches = Some(vec![]);
    let sections = Some(vec![]);
    let banner = None;

    let engineinfo = ServeItemInfo {
        creates,
        searches,
        sections,
        banner
    };

    Json(engineinfo)
}