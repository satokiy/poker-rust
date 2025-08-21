use rand::seq::SliceRandom;

use crate::domain::models::card::*;

static SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];

static NUMBERS: [CardNumber; 13] = [
    CardNumber::Ace,
    CardNumber::Two,
    CardNumber::Three,
    CardNumber::Four,
    CardNumber::Five,
    CardNumber::Six,
    CardNumber::Seven,
    CardNumber::Eight,
    CardNumber::Nine,
    CardNumber::Ten,
    CardNumber::Jack,
    CardNumber::Queen,
    CardNumber::King,
];

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for s in SUITS.iter() {
            for n in NUMBERS.iter() {
                cards.push(Card {
                    suit: *s,
                    number: *n,
                })
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&self) -> Deck {
        let mut copy = self.cards.clone();
        let mut rng = rand::rng();
        copy.shuffle(&mut rng);

        Deck { cards: copy }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn len(&mut self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.cards.is_empty()
    }
}
