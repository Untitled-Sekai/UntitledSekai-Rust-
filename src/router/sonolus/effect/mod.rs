use axum::{Router, routing::get};

mod info;
pub use info::effectinfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(effectinfo_handler))
}