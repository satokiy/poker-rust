use axum::extract::{Path, State};
/**
 * 指定した枚数のカードを引く
 */
use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::handler::card::Card;
use crate::AppState;

// TODO: game_player_handlerに移植する

#[derive(Serialize)]
pub struct DrawResponse {
    cards: Vec<Card>,
}

#[derive(Deserialize)]
pub struct DrawRequest {
    num: i32,
}

pub async fn draw(
    State(state): State<AppState>,
    Path(game_id): Path<i32>,
    Json(req): Json<DrawRequest>,
) -> impl IntoResponse {
    match state.game_service.draw_cards(game_id, req.num).await {
        Ok(cards) => Json(DrawResponse {
            cards: cards.iter().map(|c| c.to_card()).collect(),
        })
        .into_response(),
        Err(e) => e.to_response(),
    };
}
