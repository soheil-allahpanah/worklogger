mod actor;
mod dto;
mod error;
mod helpers;
mod mapper;
mod routes;
mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use infrastructure::postgres::{connect, PostgresWorklogRepository};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::router;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set at runtime (external PostgreSQL service)");
    let pool = connect(&database_url)
        .await
        .expect("failed to connect to database");

    let repo = Arc::new(PostgresWorklogRepository::new(pool));
    let state = AppState::new(repo);
    let app = router(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("invalid HOST or PORT");

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("server error");
}
