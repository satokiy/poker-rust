use crate::{
    domain::models::{card::Card, game::GamePlayer},
    error::error::AppError,
};

#[async_trait::async_trait]
pub trait GameService: Send + Sync {
    async fn create_new_game(&self) -> Result<i32, AppError>;
    async fn start_game(&self, game_id: i32) -> Result<(), AppError>;
    async fn join_game(&self, game_id: i32, player_id: i32) -> Result<(), AppError>;
    async fn get_players(&self, game_id: i32) -> Result<Vec<GamePlayer>, AppError>;

    async fn draw_cards(&self, game_id: i32, num: i32) -> Result<Vec<Card>, AppError>;
}
