use api::blackjack::Card;
use api::blackjack::DECK_OF_CARDS;
#[cfg(not(any(test, bench)))]
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    #[allow(unused_mut)]
    pub fn new() -> Self {
        let mut cards = DECK_OF_CARDS.clone();
        #[cfg(not(test))]
        thread_rng().shuffle(&mut cards);
        Self {
            cards: cards.to_vec(),
        }
    }

    pub fn draw(&mut self) -> Card {
        // Game should never get to the point where the deck is empty
        #[cfg(not(test))]
        {
            let i = thread_rng().gen_range(0, self.cards.len());
            self.cards.remove(i)
        }
        #[cfg(test)]
        {
            self.cards.pop().unwrap()
        }
    }
    pub fn export(&self) -> Vec<String> {
        self.cards.iter().map(|card| card.to_string()).collect()
    }
}

// A deck's display shouldn't reveal the contents of the deck
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck: {} cards", self.cards.len())
    }
}
