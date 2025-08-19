use std::sync::Arc;

use crate::{
    infrastructure::db::entity::deck,
    repository::{deck_repository::DeckRepository, error::RepositoryError},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, JsonValue};
use serde_json::Value;

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

    async fn pop(&self, deck_id: i32) -> Result<JsonValue, RepositoryError> {
        let deck: deck::Model = match deck::Entity::find_by_id(deck_id).one(&*self.db).await {
            Ok(Some(deck)) => deck,
            Ok(None) => return Err(RepositoryError::NotFound),
            Err(err) => return Err(RepositoryError::Internal(err.to_string())),
        };

        let mut cards: Vec<Value> = match deck.cards.as_array().cloned() {
            Some(array) => array,
            None => {
                return Err(RepositoryError::Internal(String::from(
                    "cards is not array",
                )))
            }
        };

        let card = match cards.get(0) {
            Some(card) => card.clone(),
            None => {
                return Err(RepositoryError::BadRequest(String::from(
                    "no cards left on deck.",
                )))
            }
        };

        cards.remove(0);

        let mut update_deck: deck::ActiveModel = deck.into();
        update_deck.cards = Set(Value::Array(cards));
        update_deck
            .update(&*self.db)
            .await
            .map_err(|e| RepositoryError::Internal(e.to_string()));

        Ok(card)
    }
}
