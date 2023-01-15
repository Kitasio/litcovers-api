use std::net::SocketAddr;

use axum::Server;
use router::app;

pub mod error;
pub mod overlay;
pub mod router;
pub mod settings;

pub async fn run_app(addr: SocketAddr) {
    let app = app();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
