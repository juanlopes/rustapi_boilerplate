mod config;
mod dto;
mod handlers;
mod routes;

use axum::Router;
use axum::serve;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    config::load_env();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().merge(routes::health_routes()).layer(cors);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 Server running at http://localhost:{port}");

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
    
}
