
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreatePlayerRequest {
  name: String,
}

#[derive(Serialize)]
struct CreatePlayerResponse {
  id: i32
}

pub async fn create_player(Json(req): Json<CreatePlayerRequest>) -> Json<CreatePlayerResponse> {
  
  return Json(
    CreatePlayerResponse {
      id,
    }
  )
}