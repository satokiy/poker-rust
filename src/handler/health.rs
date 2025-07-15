use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    status: String,
    message: String,
}

pub async fn health() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}
