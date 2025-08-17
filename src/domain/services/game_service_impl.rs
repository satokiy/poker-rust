use std::sync::Arc;

use crate::{
    domain::{
        models::{card::Card, deck::Deck, game::GamePlayer},
        services::game_service::GameService,
    },
    error::error::AppError,
    infrastructure::db::entity::{game_player, sea_orm_active_enums::Enum},
    repository::{
        deck_repository::DeckRepository, error::RepositoryError, game_repository::GameRepository,
        player_repository::PlayerRepository,
    },
};

pub struct GameServiceImpl {
    pub game_repository: Arc<dyn GameRepository + Send + Sync>,
    pub player_repository: Arc<dyn PlayerRepository + Send + Sync>,
    pub deck_repository: Arc<dyn DeckRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl GameService for GameServiceImpl {
    async fn create_new_game(&self) -> Result<i32, AppError> {
        let game_id = match self.game_repository.create_new().await {
            Ok(id) => id,
            Err(e) => return Err(AppError::Internal(format!("create game error: {e}"))),
        };

        let cards = Deck::new().shuffle().cards;
        let cards_json = match serde_json::to_value(&cards) {
            Ok(json) => json,
            Err(e) => return Err(AppError::Internal(format!("error create new deck: {e}"))),
        };

        self.deck_repository
            .create_new(game_id, cards_json)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(game_id)
    }

    // ゲームを開始する
    async fn start_game(&self, game_id: i32) -> Result<(), AppError> {
        match self.game_repository.find(game_id).await {
            Ok(game) => {
                if game.status != Enum::Waiting {
                    Err(AppError::bad_request(format!(
                        "game already started. game_id: {}",
                        game.id
                    )))
                } else {
                    match self.game_repository.update(game_id, Enum::InProgress).await {
                        Ok(_) => Ok(()),
                        Err(RepositoryError::NotFound) => Err(AppError::not_found()),
                        Err(err) => Err(AppError::Internal(err.to_string())),
                    }
                }
            }
            Err(err) => Err(AppError::Internal(err.to_string())),
        }
    }

    // ゲームに参加する
    async fn join_game(&self, game_id: i32, player_id: i32) -> Result<GamePlayer, AppError> {
        match self.game_repository.find(game_id).await {
            Ok(game) => {
                // 開始済みのゲームには参加できない
                if game.status != Enum::Waiting {
                    Err(AppError::bad_request(format!(
                        "game already started. game_id: {}",
                        game.id
                    )))
                } else {
                    Ok(())
                }
            }
            Err(RepositoryError::NotFound) => Err(AppError::not_found_with_msg("game not found")),
            Err(err) => Err(AppError::Internal(err.to_string())),
        }?;

        match self.player_repository.get_player(player_id).await {
            Ok(_) => Ok(()),
            Err(RepositoryError::NotFound) => Err(AppError::not_found_with_msg("player not found")),
            Err(err) => Err(AppError::Internal(err.to_string())),
        }?;

        match self
            .game_repository
            .create_game_players(game_id, vec![player_id])
            .await
        {
            Ok(players) => players
                .into_iter()
                .find(|player| player.player_id == player_id)
                .ok_or_else(|| {
                    AppError::Internal(format!("game player not found after create: {player_id}"))
                }),
            Err(e) => return Err(AppError::Internal(e.to_string())),
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
