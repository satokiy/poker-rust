use crate::{infrastructure::db::entity::player, repository::error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait PlayerRepository: Send + Sync {
    async fn insert_player(&self, name: String) -> Result<i32, RepositoryError>;
    async fn get_player(&self, id: i32) -> Result<player::Model, RepositoryError>;
}
