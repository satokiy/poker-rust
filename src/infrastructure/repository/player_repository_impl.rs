use crate::infrastructure::db::entity::player;
use crate::repository::player_repository::PlayerRepository;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::Set;

pub struct PlayerRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait::async_trait]
impl PlayerRepository for PlayerRepositoryImpl {
    async fn insert_player(&self, name: String) -> i32 {
        let player = player::ActiveModel {
            name: Set(name),
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
}
