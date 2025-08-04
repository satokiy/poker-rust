mod domain;
mod handler;
mod infrastructure;
mod repository;

// API framework routing
use axum::{routing::get, routing::post, Router};
use domain::services::player_service::PlayerService;
use domain::services::player_service_impl::PlayerServiceImpl;
use handler::draw::draw;
use handler::health::health;
use handler::player::create_player;
use infrastructure::repository::player_repository_impl::PlayerRepositoryImpl;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;

use tower_http::cors::CorsLayer;
// anyhowはRustのデフォルトのResultを扱いやすくしてくれるcrate
// https://zenn.dev/yukinarit/articles/b39cd42820f29e
use anyhow::Result;

// アプリ全体で共有したい状態やサービスをまとめた構造体。リクエストハンドラに依存を注入するために定義
#[derive(Clone)]
struct AppState {
    // Arc<T> は「複数のスレッドで安全に共有できる参照カウント付きスマートポインタ」
    // Webサーバなどで「1つのサービスを複数リクエストで共有」したいときに使う
    pub player_service: Arc<dyn PlayerService>,
}

struct Repositories {
    player_repository: PlayerRepositoryImpl,
}

struct Services {
    player_service: PlayerServiceImpl<PlayerRepositoryImpl>,
}

fn init_repositories(db: DatabaseConnection) -> Repositories {
    let player_repository = PlayerRepositoryImpl { db };

    Repositories { player_repository }
}

fn init_services(repositories: Repositories) -> Services {
    let player_service = PlayerServiceImpl {
        repository: repositories.player_repository,
    };

    Services { player_service }
}

async fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let repositories = init_repositories(db);
    let services = init_services(repositories);

    let state = AppState {
        player_service: Arc::new(services.player_service),
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
