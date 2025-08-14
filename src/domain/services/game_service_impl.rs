use crate::{
    domain::services::game_service::GameService, error::error::AppError, handler::game,
    repository::game_repository::GameRepository,
};

pub struct GameServiceImpl<R: GameRepository> {
    pub repository: R,
}

#[async_trait::async_trait]
impl<R: GameRepository + Send + Sync> GameService for GameServiceImpl<R> {
    async fn create_new_game(&self, player_ids: Vec<i32>) -> Result<i32, AppError> {
        let game_id = match self.repository.create_new_game().await {
            Ok(id) => id,
            Err(e) => return Err(AppError::Internal(format!("create game error: {e}"))),
        };

        // fkeyなので存在チェックは自動で行われる
        if let Err(e) = self.repository.create_game_players(player_ids).await {
            return Err(AppError::Internal(format!(
                "create game players error: {e}"
            )));
        }

        Ok(game_id)
    }
}
