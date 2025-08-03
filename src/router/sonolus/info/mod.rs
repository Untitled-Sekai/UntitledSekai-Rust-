use axum::{Router, routing::get};

mod info;
pub use info::info_handler;

pub fn router() -> Router {
    Router::new()
        .route("/", get(info_handler))
}