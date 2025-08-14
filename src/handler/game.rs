use crate::{handler::error::ErrorResponse, AppState};
use axum::response::{IntoResponse, Json};
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateGameRequest {
    player_ids: Vec<i32>,
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    game_id: i32,
}

pub async fn create_game(
    State(state): State<AppState>,
    Json(req): Json<CreateGameRequest>,
) -> impl IntoResponse {
    let result = state.game_service.create_new_game(req.player_ids).await;

    match result {
        Ok(id) => Json(CreateGameResponse { game_id: id }).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::from_err(e)).into_response(),
        )
            .into_response(),
    }
}
