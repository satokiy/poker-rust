mod domain;
mod error;
mod handler;
mod infrastructure;
mod repository;

use axum::routing::put;
// API framework routing
use axum::{routing::get, routing::post, Router};
use chrono::FixedOffset;
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
use crate::handler::game::{join_game, start_game};
use crate::handler::game_player::get_players;
use crate::infrastructure::repository::deck_repository_impl::DeckRepositoryImpl;
use crate::infrastructure::repository::game_repository_impl::GameRepositoryImpl;
use crate::repository::deck_repository::DeckRepository;
use crate::repository::game_repository::GameRepository;
use crate::repository::player_repository::PlayerRepository;

// アプリ全体で共有したい状態やサービスをまとめた構造体。リクエストハンドラに依存を注入するために定義
#[derive(Clone)]
struct AppState {
    // Arc<T> は「複数のスレッドで安全に共有できる参照カウント付きスマートポインタ」
    // Webサーバなどで「1つのサービスを複数リクエストで共有」したいときに使う
    pub player_service: Arc<dyn PlayerService>,
    pub game_service: Arc<dyn GameService>,
    pub timezone: FixedOffset,
}

struct Repositories {
    player_repository: Arc<dyn PlayerRepository + Send + Sync>,
    game_repository: Arc<dyn GameRepository + Send + Sync>,
    deck_repository: Arc<dyn DeckRepository + Send + Sync>,
}

struct Services {
    player_service: Arc<dyn PlayerService>,
    game_service: Arc<dyn GameService>,
}

fn init_repositories(db: Arc<DatabaseConnection>) -> Repositories {
    let player_repository = Arc::new(PlayerRepositoryImpl { db: db.clone() });
    let game_repository = Arc::new(GameRepositoryImpl { db: db.clone() });
    let deck_repository = Arc::new(DeckRepositoryImpl { db: db.clone() });

    Repositories {
        player_repository,
        game_repository,
        deck_repository,
    }
}

fn init_services(repositories: Repositories) -> Services {
    let player_service = PlayerServiceImpl {
        repository: repositories.player_repository.clone(),
    };
    let game_service = GameServiceImpl {
        game_repository: repositories.game_repository.clone(),
        player_repository: repositories.player_repository.clone(),
        deck_repository: repositories.deck_repository.clone(),
    };

    Services {
        player_service: Arc::new(player_service),
        game_service: Arc::new(game_service),
    }
}

async fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let db = Arc::new(db);
    let repositories = init_repositories(db);
    let services = init_services(repositories);

    // tokyo
    let timezone = FixedOffset::east_opt(9 * 3_600).unwrap();

    let state = AppState {
        player_service: services.player_service,
        game_service: services.game_service,
        timezone,
    };

    Router::new().nest(
        "/v1",
        Router::new()
            .route("/health", get(health))
            .nest(
                "/player",
                Router::new()
                    .route("/", post(create_player))
                    .route("/{id}", post(get_player)),
            )
            .nest(
                "/game",
                Router::new()
                    .route("/", post(create_game))
                    .route("/{game_id}/join", post(join_game))
                    .route("/{game_id}/start", put(start_game))
                    .nest(
                        "/{game_id}/player",
                        Router::new()
                            .route("/", get(get_players))
                            .route("/{player_id}/deck/draw", post(draw))
                            .route("/{player_id}/hand/exchange", post(exchange_hand))
                            .route("/{player_id}/hand/judge", post(judge_hand)),
                    ),
            )
            .layer(CorsLayer::permissive())
            .with_state(state),
    )
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
