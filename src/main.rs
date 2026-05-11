use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello world!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listener addr: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
