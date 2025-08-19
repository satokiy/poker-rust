use crate::{
    domain::models::card::{CardNumber, Suit},
    handler::card::Card,
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GamePlayer {
    id: i32,
    name: String,
    hand: Option<[Card; 5]>,
}

#[derive(Serialize)]
pub struct GetGamePlayersReponse {
    players: Vec<GamePlayer>,
}

pub async fn get_players(
    State(state): State<AppState>,
    Path(game_id): Path<i32>,
) -> impl IntoResponse {
    match state.game_service.get_players(game_id).await {
        Ok(players) => {
            let game_players = players.iter().map(|p| p.to_game_player()).collect();
            Json(GetGamePlayersReponse {
                players: game_players,
            })
            .into_response()
        }
        Err(e) => e.to_response(),
    }
}

impl crate::domain::models::game::GamePlayer {
    fn to_game_player(&self) -> GamePlayer {
        GamePlayer {
            id: self.player.id,
            name: self.player.name.clone(),
            hand: self
                .hand
                .clone()
                .map(|hand| hand.map(|card| card.to_card())),
        }
    }
}

#[derive(Deserialize)]
pub struct ExchangeHandRequest {
    cards: Vec<Card>,
}

pub async fn exchange_hand(
    State(state): State<AppState>,
    Path(player_id): Path<i32>,
    Json(req): Json<ExchangeHandRequest>,
) -> impl IntoResponse {
    todo!("not implmented");
}

#[derive(Deserialize)]
pub struct JudgeHandRequest {
    cards: Vec<Card>,
}

pub async fn judge_hand(
    State(state): State<AppState>,
    Path(player_id): Path<i32>,
    Json(req): Json<JudgeHandRequest>,
) -> impl IntoResponse {
    todo!("not implmented");
}
