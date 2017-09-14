use api::blackjack::Card;
use rand::{thread_rng, Rng};
use std::fmt;
use rayon::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}


impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![
            Card {
                name: "ACE".to_string(),
                value: 11,
            },
            Card {
                name: "TWOS".to_string(),
                value: 2,
            },
            Card {
                name: "THREES".to_string(),
                value: 3,
            },
            Card {
                name: "FOURS".to_string(),
                value: 4,
            },
            Card {
                name: "FIVES".to_string(),
                value: 5,
            },
            Card {
                name: "SIXES".to_string(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_string(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_string(),
                value: 8,
            },
            Card {
                name: "NINES".to_string(),
                value: 9,
            },
            Card {
                name: "TENS".to_string(),
                value: 10,
            },
            Card {
                name: "JACKS".to_string(),
                value: 10,
            },
            Card {
                name: "KINGS".to_string(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_string(),
                value: 10,
            },

            Card {
                name: "ACE".to_string(),
                value: 11,
            },
            Card {
                name: "TWOS".to_string(),
                value: 2,
            },
            Card {
                name: "THREES".to_string(),
                value: 3,
            },
            Card {
                name: "FOURS".to_string(),
                value: 4,
            },
            Card {
                name: "FIVES".to_string(),
                value: 5,
            },
            Card {
                name: "SIXES".to_string(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_string(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_string(),
                value: 8,
            },
            Card {
                name: "NINES".to_string(),
                value: 9,
            },
            Card {
                name: "TENS".to_string(),
                value: 10,
            },
            Card {
                name: "JACKS".to_string(),
                value: 10,
            },
            Card {
                name: "KINGS".to_string(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_string(),
                value: 10,
            },

            Card {
                name: "ACE".to_string(),
                value: 11,
            },
            Card {
                name: "TWOS".to_string(),
                value: 2,
            },
            Card {
                name: "THREES".to_string(),
                value: 3,
            },
            Card {
                name: "FOURS".to_string(),
                value: 4,
            },
            Card {
                name: "FIVES".to_string(),
                value: 5,
            },
            Card {
                name: "SIXES".to_string(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_string(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_string(),
                value: 8,
            },
            Card {
                name: "NINES".to_string(),
                value: 9,
            },
            Card {
                name: "TENS".to_string(),
                value: 10,
            },
            Card {
                name: "JACKS".to_string(),
                value: 10,
            },
            Card {
                name: "KINGS".to_string(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_string(),
                value: 10,
            },

            Card {
                name: "ACE".to_string(),
                value: 11,
            },
            Card {
                name: "TWOS".to_string(),
                value: 2,
            },
            Card {
                name: "THREES".to_string(),
                value: 3,
            },
            Card {
                name: "FOURS".to_string(),
                value: 4,
            },
            Card {
                name: "FIVES".to_string(),
                value: 5,
            },
            Card {
                name: "SIXES".to_string(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_string(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_string(),
                value: 8,
            },
            Card {
                name: "NINES".to_string(),
                value: 9,
            },
            Card {
                name: "TENS".to_string(),
                value: 10,
            },
            Card {
                name: "JACKS".to_string(),
                value: 10,
            },
            Card {
                name: "KINGS".to_string(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_string(),
                value: 10,
            },
        ];
        thread_rng().shuffle(&mut cards);
        Self { cards: cards }
    }

    pub fn draw(&mut self) -> Card {
        // Game should never get to the point where the deck is empty
        self.cards.pop().unwrap()
    }
    pub fn export(&self) -> Vec<String> {
        self.cards
            .par_iter()
            .map(|card| card.name.to_string())
            .collect()
    }
}

// A deck's display shouldn't reveal the contents of the deck
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck: {} cards", self.cards.len())
    }
}
