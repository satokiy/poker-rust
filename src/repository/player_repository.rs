use async_trait::async_trait;

#[async_trait]
pub trait PlayerRepository: Send + Sync {
  async fn insert_player(&self, name: String) -> i32;
}
