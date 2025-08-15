use crate::error::error::AppError;

#[async_trait::async_trait]
pub trait GameService: Send + Sync {
    async fn create_new_game(&self, player_ids: Vec<i32>) -> Result<i32, AppError>;
    async fn join_game(&self, game_id: i32, player_id: i32) -> Result<GamePlayer, AppError>;
}
