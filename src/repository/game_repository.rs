use crate::{
    domain::models::game::GamePlayer, infrastructure::db::entity::game,
    repository::error::RepositoryError,
};

#[async_trait::async_trait]
pub trait GameRepository {
    async fn create_new_game(&self) -> Result<i32, RepositoryError>;

    async fn find_game(&self, id: i32) -> Result<game::Model, RepositoryError>;

    async fn create_game_players(
        &self,
        game_id: i32,
        player_ids: Vec<i32>,
    ) -> Result<Vec<GamePlayer>, RepositoryError>;
}
