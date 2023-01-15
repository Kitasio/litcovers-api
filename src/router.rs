use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};

use crate::{overlay::handlers::book_cover, settings::get_config};

#[derive(Debug)]
pub struct AppState {
    pub images: Mutex<HashMap<String, Vec<u8>>>,
}

pub fn app() -> Router {
    let app_state = Arc::new(AppState {
        images: Mutex::new(HashMap::new()),
    });
    Router::new()
        .route("/health_check", get(health_check))
        .route("/overlay", post(book_cover))
        .route("/state", get(state_view))
        .with_state(app_state)
        .with_state(get_config().clone())
}

async fn state_view(State(state): State<Arc<AppState>>) -> StatusCode {
    let images = state.images.lock().unwrap();
    for (url, _bytes) in images.iter() {
        println!("url: {}", url);
    }
    StatusCode::OK
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
