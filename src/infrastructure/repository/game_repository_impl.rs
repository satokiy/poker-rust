use crate::{
    domain::models::game::GamePlayer, infrastructure::db::entity::{game, game_player, sea_orm_active_enums::Enum}, repository::{error::RepositoryError, game_repository::GameRepository}
};
use chrono::{FixedOffset, Utc};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use std::sync::Arc;

pub struct GameRepositoryImpl {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl GameRepository for GameRepositoryImpl {
    async fn create_new_game(&self) -> Result<i32, RepositoryError> {
        let now = Utc::now();
        let game = game::ActiveModel {
            status: Set(Enum::Waiting),
            created_at: Set(now.into()),
            updated_at: Set(None),
            ..Default::default()
        };

        let result = game.insert(&*self.db).await;

        match result {
            Ok(game) => Ok(game.id),
            Err(e) => {
                eprintln!("Error creating game: {e}");
                Err(RepositoryError::Internal("error create game".to_string()))
            }
        }
    }

    async fn find_game(&self, id: i32) -> Result<game::Model, RepositoryError> {
        match game::Entity::find_by_id(id).one(&*self.db).await {
            Ok(Some(game)) => Ok(game),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Internal(format!("DB error: {e}"))),
        }
    }
    async fn create_game_players(
        &self,
        game_id: i32,
        player_ids: Vec<i32>,
    ) -> Result<Vec<GamePlayer>, RepositoryError> {
        let mut players = Vec::new();
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        
        for player_id in player_ids {
            let player = game_player::ActiveModel {
                game_id: Set(game_id),
                player_id: Set(*player_id),
                hand: Set(None),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
            };

            let result = player.insert(&*self.db).await;
            match result {
                Ok(player) => {
                    players.push(
                        GamePlayer {
                            game_id:
                        }
                    )
                }
            }

        }}

        Err(RepositoryError::Internal(()));
    }
}

impl 
