use crate::domain::services::player_service::PlayerService;
use crate::error::error::AppError;
use crate::repository::player_repository::PlayerRepository;

pub struct PlayerServiceImpl<R: PlayerRepository> {
    pub repository: R,
}

#[async_trait::async_trait]
impl<R: PlayerRepository + Send + Sync> PlayerService for PlayerServiceImpl<R> {
    async fn create_player(&self, name: String) -> Result<i32, AppError> {
        match self.repository.insert_player(name).await {
            Ok(id) => Ok(id),
            Err(e) => Err(AppError::Internal(format!("error create player: {}", e))),
        }
    }
}
