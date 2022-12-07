use axum::{http::StatusCode, routing::get, Router};

use crate::settings::get_config;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .with_state(get_config().clone())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
