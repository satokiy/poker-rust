use crate::{
    infrastructure::db::entity::{game, sea_orm_active_enums::Enum},
    repository::{error::RepositoryError, game_repository::GameRepository},
};
use chrono::Utc;
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
    async fn create_game_players(&self, player_ids: Vec<i32>) -> Result<(), RepositoryError> {
        Ok(())
    }
}
