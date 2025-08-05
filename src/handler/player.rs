use crate::AppState;
use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    name: String,
}

#[derive(Serialize)]
pub struct CreatePlayerResponse {
    id: i32,
}
pub async fn create_player(
    State(state): State<AppState>,
    Json(req): Json<CreatePlayerRequest>,
) -> Json<CreatePlayerResponse> {
    let id = state.player_service.create_player(req.name).await;

    return Json(CreatePlayerResponse { id });
}
