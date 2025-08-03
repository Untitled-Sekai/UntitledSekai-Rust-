use axum::{Router, routing::get};

mod info;
pub use info::skininfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(skininfo_handler))
}