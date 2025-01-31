use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello from Service A!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5001));
    println!("Service A running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // axum::serve::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
