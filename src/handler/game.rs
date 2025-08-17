use crate::{handler::error::ErrorResponse, AppState};
use axum::response::{IntoResponse, Json};
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateGameResponse {
    game_id: i32,
}

pub async fn create_game(State(state): State<AppState>) -> impl IntoResponse {
    let result = state.game_service.create_new_game().await;

    match result {
        Ok(id) => Json(CreateGameResponse { game_id: id }).into_response(),
        Err(e) => e.to_response(),
    }
}

#[derive(Deserialize)]
pub struct JoinGameRequest {
    game_id: i32,
    player_id: i32,
}

#[derive(Deserialize)]
pub struct StartGameRequest {
    game_id: i32,
}

pub async fn join_game(
    State(state): State<AppState>,
    Json(req): Json<JoinGameRequest>,
) -> impl IntoResponse {
    let result = state
        .game_service
        .join_game(req.game_id, req.player_id)
        .await;
    match result {
        Ok(_) => Json(()).into_response(),
        Err(e) => e.to_response(),
    }
}

pub async fn start_game(
    State(state): State<AppState>,
    Json(req): Json<StartGameRequest>,
) -> impl IntoResponse {
    let result = state.game_service.start_game(req.game_id).await;

    match result {
        Ok(_) => Json(()).into_response(),
        Err(e) => e.to_response(),
    }
}
