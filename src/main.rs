use axum::Server;
use litcovers_api::router::app;

#[tokio::main]
async fn main() {
    let app = app();
    let addr = "[::]:8080".parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
