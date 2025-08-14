use crate::repository::error::RepositoryError;

#[async_trait::async_trait]
pub trait GameRepository {
    async fn create_new_game(&self) -> Result<i32, RepositoryError>;
    async fn create_game_players(&self, player_ids: Vec<i32>) -> Result<(), RepositoryError>;
}
