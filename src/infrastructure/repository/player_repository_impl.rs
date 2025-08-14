use crate::infrastructure::db::entity::player;
use crate::repository::error::RepositoryError;
use crate::repository::player_repository::PlayerRepository;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;

pub struct PlayerRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait::async_trait]
impl PlayerRepository for PlayerRepositoryImpl {
    async fn insert_player(&self, name: String) -> Result<i32, RepositoryError> {
        let now = Utc::now();
        let player = player::ActiveModel {
            name: Set(name),
            created_at: Set(now.into()),
            updated_at: Set(None),
            ..Default::default()
        };

        let res = player.insert(&self.db).await;

        match res {
            Ok(player) => Ok(player.id),
            Err(e) => {
                eprintln!("Error inserting player: {e}");
                Err(RepositoryError::Internal(String::from(
                    "Error inserting player",
                )))
            }
        }
    }

    async fn get_player(&self, id: i32) -> Result<player::Model, RepositoryError> {
        let res = player::Entity::find_by_id(id).one(&self.db).await;

        match res {
            Ok(Some(player)) => Ok(player),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Internal(format!("DB error: {e}"))),
        }
    }
}
