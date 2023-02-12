use std::net::SocketAddr;

use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "hello"
}
