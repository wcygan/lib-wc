use axum::{routing::get, Router, Server};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "hello"
}
