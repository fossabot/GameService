use api::blackjack::Card;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[allow(dead_code)]
impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![
            Card {
                name: "ACE".to_owned(),
                value: 11,
            },
            Card {
                name: "TWOS".to_owned(),
                value: 2,
            },
            Card {
                name: "THREES".to_owned(),
                value: 3,
            },
            Card {
                name: "FOURS".to_owned(),
                value: 4,
            },
            Card {
                name: "FIVES".to_owned(),
                value: 5,
            },
            Card {
                name: "SIXES".to_owned(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_owned(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_owned(),
                value: 8,
            },
            Card {
                name: "NINES".to_owned(),
                value: 9,
            },
            Card {
                name: "TENS".to_owned(),
                value: 10,
            },
            Card {
                name: "JACKS".to_owned(),
                value: 10,
            },
            Card {
                name: "KINGS".to_owned(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_owned(),
                value: 10,
            },

            Card {
                name: "ACE".to_owned(),
                value: 11,
            },
            Card {
                name: "TWOS".to_owned(),
                value: 2,
            },
            Card {
                name: "THREES".to_owned(),
                value: 3,
            },
            Card {
                name: "FOURS".to_owned(),
                value: 4,
            },
            Card {
                name: "FIVES".to_owned(),
                value: 5,
            },
            Card {
                name: "SIXES".to_owned(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_owned(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_owned(),
                value: 8,
            },
            Card {
                name: "NINES".to_owned(),
                value: 9,
            },
            Card {
                name: "TENS".to_owned(),
                value: 10,
            },
            Card {
                name: "JACKS".to_owned(),
                value: 10,
            },
            Card {
                name: "KINGS".to_owned(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_owned(),
                value: 10,
            },

            Card {
                name: "ACE".to_owned(),
                value: 11,
            },
            Card {
                name: "TWOS".to_owned(),
                value: 2,
            },
            Card {
                name: "THREES".to_owned(),
                value: 3,
            },
            Card {
                name: "FOURS".to_owned(),
                value: 4,
            },
            Card {
                name: "FIVES".to_owned(),
                value: 5,
            },
            Card {
                name: "SIXES".to_owned(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_owned(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_owned(),
                value: 8,
            },
            Card {
                name: "NINES".to_owned(),
                value: 9,
            },
            Card {
                name: "TENS".to_owned(),
                value: 10,
            },
            Card {
                name: "JACKS".to_owned(),
                value: 10,
            },
            Card {
                name: "KINGS".to_owned(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_owned(),
                value: 10,
            },

            Card {
                name: "ACE".to_owned(),
                value: 11,
            },
            Card {
                name: "TWOS".to_owned(),
                value: 2,
            },
            Card {
                name: "THREES".to_owned(),
                value: 3,
            },
            Card {
                name: "FOURS".to_owned(),
                value: 4,
            },
            Card {
                name: "FIVES".to_owned(),
                value: 5,
            },
            Card {
                name: "SIXES".to_owned(),
                value: 6,
            },
            Card {
                name: "SEVENS".to_owned(),
                value: 7,
            },
            Card {
                name: "EIGHTS".to_owned(),
                value: 8,
            },
            Card {
                name: "NINES".to_owned(),
                value: 9,
            },
            Card {
                name: "TENS".to_owned(),
                value: 10,
            },
            Card {
                name: "JACKS".to_owned(),
                value: 10,
            },
            Card {
                name: "KINGS".to_owned(),
                value: 10,
            },
            Card {
                name: "QUEENS".to_owned(),
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

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.cards)
    }
}

// A deck's display shouldn't reveal the contents of the deck
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck: {} cards", self.cards.len())
    }
}
