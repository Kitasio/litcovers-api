use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use litcovers_api::router::app;
use tower::ServiceExt; // for `oneshot` and `ready`

#[tokio::test]
async fn health_check_works() {
    let app = app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
