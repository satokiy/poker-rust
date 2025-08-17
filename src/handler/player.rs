use crate::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    name: String,
}

#[derive(Serialize)]
pub struct CreatePlayerResponse {
    id: i32,
}

#[derive(Serialize)]
pub struct GetPlayerResponse {
    id: i32,
    name: String,
}

pub async fn create_player(
    State(state): State<AppState>,
    Json(req): Json<CreatePlayerRequest>,
) -> impl IntoResponse {
    let result = state.player_service.create_player(req.name).await;

    match result {
        Ok(id) => Json(CreatePlayerResponse { id }).into_response(),
        Err(e) => e.to_response(),
    }
}

pub async fn get_player(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let res = state.player_service.get_player(id).await;
    match res {
        Ok(player) => Json(GetPlayerResponse {
            id: player.id,
            name: player.name,
        })
        .into_response(),
        Err(e) => e.to_response(),
    }
}
