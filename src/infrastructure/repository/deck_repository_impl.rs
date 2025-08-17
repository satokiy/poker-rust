use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{
    infrastructure::db::entity::deck,
    repository::{deck_repository::DeckRepository, error::RepositoryError},
};

pub struct DeckRepositoryImpl {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl DeckRepository for DeckRepositoryImpl {
    async fn create_new(
        &self,
        game_id: i32,
        cards: serde_json::Value,
    ) -> Result<i32, RepositoryError> {
        let deck = deck::ActiveModel {
            game_id: Set(game_id),
            cards: Set(cards),
            ..Default::default()
        };

        let result = deck.insert(&*self.db).await?;

        Ok(result.id)
    }
}
