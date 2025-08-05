use crate::domain::services::player_service::PlayerService;
use crate::repository::player_repository::PlayerRepository;

pub struct PlayerServiceImpl<R: PlayerRepository> {
    pub repository: R,
}

#[async_trait::async_trait]
impl<R: PlayerRepository + Send + Sync> PlayerService for PlayerServiceImpl<R> {
    async fn create_player(&self, name: String) -> i32 {
        self.repository.insert_player(name).await
    }
}
