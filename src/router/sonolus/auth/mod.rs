use axum::{Router, routing::post};

mod auth;

pub use auth::auth_handler;

pub fn router() -> Router {
    Router::new()
        .route("/", post(auth_handler))
}