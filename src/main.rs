use axum::{http::StatusCode, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(health_check));
    let addr = "[::]:8080".parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
