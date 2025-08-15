use crate::domain::models::card::Card;

pub struct GamePlayer {
    pub game_id: i32,
    pub player_id: i32,
    pub hand: Option<[Card; 5]>, // gameスタート時はNone
}
