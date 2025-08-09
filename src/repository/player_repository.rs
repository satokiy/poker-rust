use async_trait::async_trait;
use crate::infrastructure::db::entity::player;

#[async_trait]
pub trait PlayerRepository: Send + Sync {
    async fn insert_player(&self, name: String) -> i32;
    async fn get_player(&self, id: i32) -> player::Model;
}
