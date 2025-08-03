use axum::{Router, routing::get};

mod info;
pub use info::levelinfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(levelinfo_handler))
}