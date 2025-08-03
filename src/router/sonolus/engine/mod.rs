use axum::{Router, routing::get};

mod info;
pub use info::engineinfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(engineinfo_handler))
}