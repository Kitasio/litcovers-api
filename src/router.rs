use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use crate::{overlay::handlers::book_cover, settings::get_config};

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/overlay", post(book_cover))
        .with_state(get_config().clone())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
