use api::blackjack::Card;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[allow(dead_code)]
impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::with_capacity(5) }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn score(&self) -> u64 {
        let mut ace_count = 08;
        let mut total = 0u64;
        for card in &self.cards {
            if card.name == "Ace" {
                ace_count += 1
            } else {
                total += card.value as u64;
            }
        }
        if ace_count >= 1 {
            if total < 10 {
                total += 11
            } else {
                total += 1
            };
            ace_count -= 1;
        };
        for _ace in 0u8..ace_count {
            total += 1
        }
        total
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score: {}", self.score())
    }
}
