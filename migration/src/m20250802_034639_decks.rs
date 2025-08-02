use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Deck::Table)
                    .if_not_exists()
                    .col(pk_auto(Deck::Id))
                    .col(integer(Deck::GameId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_deck_game")
                            .from(Deck::Table, Deck::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Deck::Cards).json_binary().not_null())
                    .col(timestamp_with_time_zone(Deck::CreatedAt))
                    .col(timestamp_with_time_zone_null(Deck::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Deck::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Deck {
    Table,
    Id,
    GameId,
    Cards,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
}
