use crate::domain::models::card::Card as CardModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub suit: String,
    pub number: i32,
}

impl From<&CardModel> for Card {
    fn from(card: &CardModel) -> Self {
        Card {
            suit: format!("{:?}", card.suit),
            number: card.number as i32,
        }
    }
}

impl CardModel {
    pub fn to_card(&self) -> Card {
        Card {
            suit: self.suit.to_string(),
            number: self.number.as_int(),
        }
    }
}
