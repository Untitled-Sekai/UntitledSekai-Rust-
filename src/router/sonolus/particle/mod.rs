use axum::{Router, routing::get};

mod info;
pub use info::particleinfo_handler;

pub fn router() -> Router {
    Router::new()
        .route("/info", get(particleinfo_handler))
}