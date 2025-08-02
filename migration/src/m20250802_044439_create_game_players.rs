use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GamePlayer::Table)
                    .if_not_exists()
                    .col(integer(GamePlayer::GameId).not_null())
                    .col(integer(GamePlayer::PlayerId).not_null())
                    .col(ColumnDef::new(GamePlayer::Hand).json_binary())
                    .col(timestamp_with_time_zone(GamePlayer::CreatedAt))
                    .col(timestamp_with_time_zone_null(GamePlayer::UpdatedAt))
                    .primary_key(
                        Index::create()
                            .col(GamePlayer::GameId)
                            .col(GamePlayer::PlayerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_player_game_id")
                            .from(GamePlayer::Table, GamePlayer::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_player_player_id")
                            .from(GamePlayer::Table, GamePlayer::PlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GamePlayer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GamePlayer {
    Table,
    GameId,
    PlayerId,
    Hand,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
}
