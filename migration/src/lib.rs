pub use sea_orm_migration::prelude::*;

mod m20250802_031125_create_games;
mod m20250802_034639_decks;
mod m20250802_044138_create_players;
mod m20250802_044439_create_game_players;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250802_031125_create_games::Migration),
            Box::new(m20250802_034639_decks::Migration),
            Box::new(m20250802_044138_create_players::Migration),
            Box::new(m20250802_044439_create_game_players::Migration),
        ]
    }
}
