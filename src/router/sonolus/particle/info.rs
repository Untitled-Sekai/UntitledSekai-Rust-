use axum::response::Json;
use crate::model::info::ServeItemInfo;

pub async fn particleinfo_handler() -> Json<ServeItemInfo> {
    let creates = Some(vec![]);
    let searches = Some(vec![]);
    let sections = Some(vec![]);
    let banner = None;

    let particleinfo = ServeItemInfo {
        creates,
        searches,
        sections,
        banner
    };

    Json(particleinfo)
}