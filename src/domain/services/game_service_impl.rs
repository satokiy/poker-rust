use crate::{
    domain::{
        models::{card::Card, game::GamePlayer},
        services::game_service::GameService,
    },
    error::error::AppError,
    handler::game,
    infrastructure::db::entity::game_player,
    repository::{
        error::RepositoryError, game_repository::GameRepository,
        player_repository::PlayerRepository,
    },
};

pub struct GameServiceImpl<GR: GameRepository, PR: PlayerRepository> {
    pub game_repository: GR,
    pub player_repository: PR,
}

#[async_trait::async_trait]
impl<R: GameRepository + Send + Sync> GameService for GameServiceImpl<GR, PR> {
    async fn create_new_game(&self, player_ids: Vec<i32>) -> Result<i32, AppError> {
        let game_id = match self.game_repository.create_new_game().await {
            Ok(id) => id,
            Err(e) => return Err(AppError::Internal(format!("create game error: {e}"))),
        };

        // TODO: deckの初期化

        Ok(game_id)
    }

    async fn join_game(&self, game_id: i32, player_id: i32) -> Result<GamePlayer, AppError> {
        match self.game_repository.find_game(game_id).await {
            Err(RepositoryError::NotFound) => Err(AppError::NotFound()),
            Err(RepositoryError::Internal(err)) => Err(AppError::Internal(err)),
            Ok(game) => Ok(()),
        };

        match self.player_repository.get_player(player_id).await {
            Err(RepositoryError::NotFound) => Err(AppError::NotFound()),
            Err(RepositoryError::Internal(err)) => Err(AppError::Internal(err)),
            Ok(player) => Ok(()),
        };

        // TODO: 複数返ってくるので自分を探す
        match self
            .game_repository
            .create_game_players(game_id, [player_id])
            .await
        {
            Ok(players) => players,
            Err(e) => return Err(AppError::Internal(e)),
        }
    }
}

impl game_player::Model {
    fn to_game_player(&self) -> GamePlayer {
        let hand = match &self.hand {
            Some(json) => serde_json::from_value::<[Card; 5]>(json.clone()).ok(),
            None => None,
        };

        GamePlayer {
            game_id: self.game_id,
            player_id: self.player_id,
            hand,
        }
    }
}
