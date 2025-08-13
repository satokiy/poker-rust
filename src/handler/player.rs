use crate::handler::error::ErrorResponse;
use crate::AppState;

use axum::extract::State;
use axum::http::StatusCode;
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

pub async fn create_player(
    State(state): State<AppState>,
    Json(req): Json<CreatePlayerRequest>,
) -> impl IntoResponse {
    let res = state.player_service.create_player(req.name).await;

    match res {
        Ok(id) => Json(CreatePlayerResponse { id }).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::from_err(e)).into_response(),
        )
            .into_response(),
    }
}
