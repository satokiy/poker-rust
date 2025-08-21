use crate::{
    infrastructure::db::entity::game_player,
    repository::{error::RepositoryError, game_player_repository::GamePlayerRepository},
};
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect, SelectColumns};
use std::sync::Arc;

pub struct GamePlayerRepositoryImpl {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl GamePlayerRepository for GamePlayerRepositoryImpl {
    async fn get_game_player(
        &self,
        game_id: i32,
        player_id: i32,
    ) -> Result<game_player::Model, RepositoryError> {
        match game_player::Entity::find()
            .filter(game_player::Column::GameId.eq(game_id))
            .filter(game_player::Column::PlayerId.eq(player_id))
            .one(&*self.db)
            .await
        {
            Ok(Some(game_player)) => Ok(game_player),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Internal(format!("DB error: {e}"))),
        }
    }
}
