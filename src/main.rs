use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;

mod routes;
use routes::{airdrop, get_balance};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/airdrop", post(airdrop))
        .route("/balance/:address", get(get_balance));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on :  http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
