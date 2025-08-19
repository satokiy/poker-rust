use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Spade => "Spade",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for Suit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Heart" => Ok(Suit::Heart),
            "Diamond" => Ok(Suit::Diamond),
            "Club" => Ok(Suit::Club),
            "Spade" => Ok(Suit::Spade),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardNumber {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl CardNumber {
    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i64> for CardNumber {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CardNumber::Ace),
            2 => Ok(CardNumber::Two),
            3 => Ok(CardNumber::Three),
            4 => Ok(CardNumber::Four),
            5 => Ok(CardNumber::Five),
            6 => Ok(CardNumber::Six),
            7 => Ok(CardNumber::Seven),
            8 => Ok(CardNumber::Eight),
            9 => Ok(CardNumber::Nine),
            10 => Ok(CardNumber::Ten),
            11 => Ok(CardNumber::Jack),
            12 => Ok(CardNumber::Queen),
            13 => Ok(CardNumber::King),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNumber,
}
