use axum::{Router, routing::get};

mod info;
pub use info::backgroundinfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(backgroundinfo_handler))
}