use crate::{
    domain::models::game::GamePlayer,
    infrastructure::db::entity::{game, sea_orm_active_enums::Enum},
    repository::error::RepositoryError,
};

#[async_trait::async_trait]
pub trait GameRepository {
    async fn create_new(&self) -> Result<i32, RepositoryError>;

    async fn find(&self, id: i32) -> Result<game::Model, RepositoryError>;

    async fn update(&self, id: i32, status: Enum) -> Result<(), RepositoryError>;

    async fn create_game_players(
        &self,
        game_id: i32,
        player_ids: Vec<i32>,
    ) -> Result<Vec<GamePlayer>, RepositoryError>;
}
