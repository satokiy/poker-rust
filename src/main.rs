mod domain;
mod handler;
mod repository;

// API framework routing
use std::sync::Arc;
use axum::{routing::get, routing::post, Router};
use handler::draw::draw;
use handler::health::health;
use handler::player::create_player;
use domain::services::player_service::PlayerService;
use domain::services::player_service::PlayerServiceImpl;
use std::env;

use tower_http::cors::CorsLayer;
// anyhowはRustのデフォルトのResultを扱いやすくしてくれるcrate
// https://zenn.dev/yukinarit/articles/b39cd42820f29e
use anyhow::Result;

#[derive(Clone)]
struct AppState {
    pub player_service: Arc<dyn PlayerService>
}

async fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let service = PlayerServiceImpl {}
    let state = AppState {
        player_service: Arc::new(service),
    };

    Router::new()
        .route("/health", get(health))
        .route("/v1/decks/draw", post(draw))
        .route("/v1/player", post(create_player))
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
