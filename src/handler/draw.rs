use crate::domain::models::card::Card;
use crate::domain::models::deck::Deck;
/**
 * 指定した枚数のカードを引く
 */
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DrawResponse {
    status: String,
    cards: Vec<CardResponse>,
}

#[derive(Serialize)]
pub struct CardResponse {
    suit: String,
    number: u8,
}

#[derive(Deserialize)]
pub struct DrawRequest {
    num: i32,
}

// From trait
impl From<&Card> for CardResponse {
    fn from(card: &Card) -> Self {
        CardResponse {
            suit: format!("{:?}", card.suit),
            number: card.number as u8,
        }
    }
}

pub async fn draw(Json(req): Json<DrawRequest>) -> Json<DrawResponse> {
    let mut deck = Deck::new();
    let mut cards: Vec<Card> = Vec::new();

    for _ in 0..req.num {
        // デッキに指定した枚数のカードがあるとは限らない
        if let Some(card) = deck.draw() {
            cards.push(card);
        }
    }
    let result: Vec<CardResponse> = cards.iter().map(CardResponse::from).collect();

    Json(DrawResponse {
        status: "ok".to_string(),
        cards: result,
    })
}
