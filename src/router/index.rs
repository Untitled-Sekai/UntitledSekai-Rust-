use axum::Router;

pub fn sonolus_router() -> Router {
    Router::new()
        .nest("/levels", crate::router::sonolus::level::router())
        .nest("/info", crate::router::sonolus::info::router())
        .nest("/backgrounds", crate::router::sonolus::background::router())
        .nest("/skins", crate::router::sonolus::skin::router())
        .nest("/particles", crate::router::sonolus::particle::router())
        .nest("/engines", crate::router::sonolus::engine::router())
        .nest("/effects", crate::router::sonolus::effect::router())
}