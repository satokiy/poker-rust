use crate::domain::models::{card::Card, player::Player};

pub struct GamePlayer {
    pub game_id: i32,
    pub player: Player,
    pub hand: Option<[Card; 5]>, // gameスタート時はNone
}
