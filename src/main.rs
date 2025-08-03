use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
mod model;
mod router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "test" }))
        // ファイルを提供するルート
        .nest("/assets", router::assets::assets_router())
        // Sonolusのルート
        .nest("/sonolus", router::index::sonolus_router());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}