// assetsを提供

use axum::Router;
use tower_http::services::ServeDir;

pub fn assets_router() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new("assets")
    )
}