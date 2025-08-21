use std::sync::Arc;

use crate::domain::models::player::Player;
use crate::domain::services::player_service::PlayerService;
use crate::error::error::AppError;
use crate::infrastructure::db::entity::player;
use crate::repository::error::RepositoryError;
use crate::repository::player_repository::PlayerRepository;

pub struct PlayerServiceImpl {
    pub repository: Arc<dyn PlayerRepository>,
}

#[async_trait::async_trait]
impl PlayerService for PlayerServiceImpl {
    async fn create_player(&self, name: String) -> Result<i32, AppError> {
        match self.repository.insert_player(name).await {
            Ok(id) => Ok(id),
            Err(e) => Err(AppError::Internal(format!("error create player: {e}"))),
        }
    }

    async fn get_player(&self, id: i32) -> Result<Player, AppError> {
        match self.repository.get_player(id).await {
            Ok(player) => Ok(player.to_player()),
            Err(RepositoryError::NotFound) => Err(AppError::not_found()),
            Err(e) => Err(AppError::Internal(format!("error: {e}"))),
        }
    }
}

impl player::Model {
    fn to_player(&self) -> Player {
        Player {
            id: self.id,
            name: self.name.clone(),
        }
    }
}
