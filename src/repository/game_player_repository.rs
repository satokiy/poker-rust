use crate::{infrastructure::db::entity::game_player, repository::error::RepositoryError};

#[async_trait::async_trait]
pub trait GamePlayerRepository {
    async fn get_game_player(
        &self,
        game_id: i32,
        player_id: i32,
    ) -> Result<game_player::Model, RepositoryError>;
}
