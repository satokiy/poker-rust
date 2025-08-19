use sea_orm::JsonValue;

use crate::repository::error::RepositoryError;

#[async_trait::async_trait]
pub trait DeckRepository {
    async fn create_new(
        &self,
        game_id: i32,
        cards: serde_json::Value,
    ) -> Result<i32, RepositoryError>;

    async fn pop(&self, deck_id: i32) -> Result<JsonValue, RepositoryError>;
}
