use litcovers_api::run_app;

#[tokio::main]
async fn main() {
    let addr = "[::]:8080".parse().unwrap();
    run_app(addr).await
}
