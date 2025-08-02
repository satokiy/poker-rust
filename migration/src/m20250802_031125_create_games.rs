use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_type(
                Type::create()
                    .as_enum(GameStatusType::Enum)
                    .values(vec!["waiting", "in_progress", "finished"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(
                        ColumnDef::new(Game::Status)
                            .custom(GameStatusType::Enum)
                            .not_null(),
                    )
                    .col(timestamp_with_time_zone(Game::CreatedAt).not_null())
                    .col(timestamp_with_time_zone_null(Game::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(GameStatusType::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GameStatusType {
    Enum,
}
