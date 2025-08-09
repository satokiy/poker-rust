use crate::error::error::AppError;
use crate::infrastructure::db::entity::player;
use crate::repository::player_repository::PlayerRepository;
use crate::error::error;
use anyhow::Error;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::Iden;
use sea_orm::Set;
use chrono::Utc;
use sea_orm::EntityTrait;

pub struct PlayerRepositoryImpl {
    pub db: DatabaseConnection,
}


#[async_trait::async_trait]
impl PlayerRepository for PlayerRepositoryImpl {
    async fn insert_player(&self, name: String) -> i32 {
        let now = Utc::now();
        let player = player::ActiveModel {
            name: Set(name),
            created_at: Set(now.into()),
            updated_at: Set(None),
            ..Default::default()
        };

        let res: Result<player::Model, sea_orm::DbErr> = player.insert(&self.db).await;
        match res {
            Ok(player) => player.id,
            Err(e) => {
                eprintln!("Error inserting player: {}", e);
                -1 // or handle the error as needed
            }
        }
    }

    async fn get_player(&self, id: i32) -> Result<player::Model, AppError> {
        let res = player::Entity::find_by_id(id).one(&self.db).await?;
        match res {
            Some(player) => Ok(player),
            None => Err(error::AppError::NotFound("not found".to_string()))
        }
    }
}
