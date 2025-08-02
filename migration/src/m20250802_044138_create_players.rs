use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(pk_auto(Player::Id))
                    .col(string(Player::Name))
                    .col(timestamp_with_time_zone(Player::CreatedAt))
                    .col(timestamp_with_time_zone_null(Player::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
