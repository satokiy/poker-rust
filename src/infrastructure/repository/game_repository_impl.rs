use crate::{
    domain::models::game::GamePlayer,
    infrastructure::db::entity::{game, game_player, player, sea_orm_active_enums::Enum},
    repository::{error::RepositoryError, game_repository::GameRepository},
};
use chrono::{FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect, SelectColumns};
use std::sync::Arc;

pub struct GameRepositoryImpl {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl GameRepository for GameRepositoryImpl {
    async fn create_new(&self) -> Result<i32, RepositoryError> {
        let now = Utc::now();
        let game = game::ActiveModel {
            status: Set(Enum::Waiting),
            created_at: Set(now.into()),
            updated_at: Set(None),
            ..Default::default()
        };

        let result = game.insert(&*self.db).await;

        match result {
            Ok(game) => Ok(game.id),
            Err(e) => {
                eprintln!("Error creating game: {e}");
                Err(RepositoryError::Internal("error create game".to_string()))
            }
        }
    }

    async fn find(&self, id: i32) -> Result<game::Model, RepositoryError> {
        match game::Entity::find_by_id(id).one(&*self.db).await {
            Ok(Some(game)) => Ok(game),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Internal(format!("DB error: {e}"))),
        }
    }

    async fn update(&self, id: i32, status: Enum) -> Result<(), RepositoryError> {
        let result = game::Entity::update_many()
            .col_expr(game::Column::Status, status.into())
            .filter(game::Column::Id.eq(id))
            .exec(&*self.db)
            .await?;

        if result.rows_affected == 0 {
            Err(RepositoryError::NotFound)
        } else {
            Ok(())
        }
    }

    async fn create_game_players(
        &self,
        game_id: i32,
        player_ids: Vec<i32>,
    ) -> Result<Vec<GamePlayer>, RepositoryError> {
        let exist_ids = player::Entity::find()
            .select_only()
            .select_column(player::Column::Id)
            .filter(player::Column::Id.is_in(player_ids.clone()))
            .into_tuple::<i32>()
            .all(&*self.db)
            .await?;

        if player_ids.len() != exist_ids.len() {
            return Err(RepositoryError::NotFound);
        }

        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let model_players: Vec<game_player::ActiveModel> = player_ids
            .iter()
            .map(|id| game_player::ActiveModel {
                game_id: Set(game_id),
                player_id: Set(*id),
                hand: Set(None), // 参加時点では手札はなし
                created_at: Set(now),
                updated_at: Set(now.into()),
            })
            .collect();

        game_player::Entity::insert_many(model_players)
            .exec(&*self.db)
            .await?;

        let players = player_ids
            .into_iter()
            .map(|player_id| GamePlayer {
                game_id,
                player_id,
                hand: None,
            })
            .collect();

        Ok(players)
    }
}
