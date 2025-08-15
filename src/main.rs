mod domain;
mod error;
mod handler;
mod infrastructure;
mod repository;

// API framework routing
use axum::{routing::get, routing::post, Router};
use domain::services::player_service::PlayerService;
use domain::services::player_service_impl::PlayerServiceImpl;
use handler::draw::draw;
use handler::game::create_game;
use handler::health::health;
use handler::player::{create_player, get_player};
use infrastructure::repository::player_repository_impl::PlayerRepositoryImpl;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;

use tower_http::cors::CorsLayer;
// anyhowはRustのデフォルトのResultを扱いやすくしてくれるcrate
// https://zenn.dev/yukinarit/articles/b39cd42820f29e
use anyhow::Result;

use crate::domain::services::game_service::GameService;
use crate::domain::services::game_service_impl::GameServiceImpl;
use crate::infrastructure::repository::game_repository_impl::GameRepositoryImpl;

// アプリ全体で共有したい状態やサービスをまとめた構造体。リクエストハンドラに依存を注入するために定義
#[derive(Clone)]
struct AppState {
    // Arc<T> は「複数のスレッドで安全に共有できる参照カウント付きスマートポインタ」
    // Webサーバなどで「1つのサービスを複数リクエストで共有」したいときに使う
    pub player_service: Arc<dyn PlayerService>,
    pub game_service: Arc<dyn GameService>,
}

struct Repositories {
    player_repository: PlayerRepositoryImpl,
    game_repository: GameRepositoryImpl,
}

struct Services {
    player_service: PlayerServiceImpl<PlayerRepositoryImpl>,
    game_service: GameServiceImpl<GameRepositoryImpl, PlayerRepositoryImpl>,
}

fn init_repositories(db: Arc<DatabaseConnection>) -> Repositories {
    let player_repository = PlayerRepositoryImpl { db: db.clone() };
    let game_repository = GameRepositoryImpl { db: db.clone() };

    Repositories {
        player_repository,
        game_repository,
    }
}

fn init_services(repositories: Repositories) -> Services {
    let player_service = PlayerServiceImpl {
        repository: repositories.player_repository,
    };
    let game_service = GameServiceImpl {
        game_repository: repositories.game_repository,
        player_repository: repositories.player_repository,
    };

    Services {
        player_service,
        game_service,
    }
}

async fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let db = Arc::new(db);
    let repositories = init_repositories(db);
    let services = init_services(repositories);

    let state = AppState {
        player_service: Arc::new(services.player_service),
        game_service: Arc::new(services.game_service),
    };

    Router::new()
        .route("/health", get(health))
        .route("/v1/decks/draw", post(draw))
        .route("/v1/player", post(create_player))
        .route("/v1/player/{id}", get(get_player))
        .route("/v1/game", post(create_game))
        .route("/v1/game/join", post(join_game))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://poker_user:poker_password@localhost:6432/poker_db".to_string()
    });

    let db = sea_orm::Database::connect(&database_url).await?;

    let app = create_app(db).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server starting on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
