mod app_state;
mod configuration;
mod routes;
mod types;
mod auth_user;

use routes::create_app;
use std::net::SocketAddr;

pub async fn run() {
    let app = create_app().await;

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
