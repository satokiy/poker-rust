use crate::{domain::models::player::Player, error::error::AppError};

#[async_trait::async_trait]
pub trait PlayerService: Send + Sync {
    async fn create_player(&self, name: String) -> Result<i32, AppError>;
    async fn get_player(&self, id: i32) -> Result<Player, AppError>;
}
