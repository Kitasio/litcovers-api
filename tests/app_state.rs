use std::time::Instant;

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use litcovers_api::{
    overlay::handlers::BookCoverParams,
    overlay::image::{BlendMode, PositionType},
    router::app,
};
use tower::ServiceExt;

#[tokio::test]
async fn req_from_state_is_faster() {
    let app = app();
    let body_data = BookCoverParams {
        author_font: "Stig.ttf".to_string(),
        author: "Prison Mike".to_string(),
        author_position: PositionType::TopCenter,
        title_font: "Stig.ttf".to_string(),
        title: "Harry Potter and other people".to_string(),
        title_position: PositionType::BottomCenter,
        blend_mode: BlendMode::Overlay,
        alfa: 3.0,
        image_url: "https://replicate.delivery/pbxt/pX5B4V8QzvKFBBk7CHm788FQZKeQXvO8RbhfGNLXpIbYcZUQA/out-0.png".to_string(),
        line_length: 16,
    };

    let start = Instant::now();
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/overlay")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&body_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let duration1 = start.elapsed();

    assert_eq!(response1.status(), StatusCode::OK);

    let start = Instant::now();
    let response2 = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/overlay")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&body_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let duration2 = start.elapsed();

    assert_eq!(response2.status(), StatusCode::OK);
    assert!(duration2 < duration1);
}
