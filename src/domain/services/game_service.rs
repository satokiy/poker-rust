use crate::error::error::AppError;

#[async_trait::async_trait]
pub trait GameService: Send + Sync {
    async fn create_new_game(&self, player_ids: Vec<i32>) -> Result<i32, AppError>;
}
