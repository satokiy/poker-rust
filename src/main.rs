mod handler;
// API framework routing
use axum::{routing::get, Router};
use handler::health::health;
use std::env;

use tower_http::cors::CorsLayer;
// anyhowはRustのデフォルトのResultを扱いやすくしてくれるcrate
// https://zenn.dev/yukinarit/articles/b39cd42820f29e
use anyhow::Result;

#[derive(Clone)]
struct AppState {
    db: sea_orm::DatabaseConnection,
}

async fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let state = AppState { db };

    Router::new()
        .route("/health", get(health))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://poker_user:poker_password@localhost:5432/poker_db".to_string()
    });

    let db = sea_orm::Database::connect(&database_url).await?;

    let app = create_app(db).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server starting on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
