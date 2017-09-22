#[cfg(test)]
use std::fmt;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: &'static str,
    pub value: u8,
}

const NON_ROYALTY_CARDS: [&'static str; 10] = [
    "ACE",
    "TWOS",
    "THREES",
    "FOURS",
    "FIVES",
    "SIXES",
    "SEVENS",
    "EIGHTS",
    "NINES",
    "TENS",
];

const ROYALTY_CARDS: [&'static str; 3] = ["JACKS", "KINGS", "QUEENS"];

impl Card {
    // Used to restore from DB
    pub fn new(card_name: &str) -> Result<Self, ()> {
        match NON_ROYALTY_CARDS.iter().position(|&r| r == card_name) {
            Some(position) => Ok(Card {
                name: NON_ROYALTY_CARDS[position],
                value: position as u8 + 1u8,
            }),
            None => match ROYALTY_CARDS.iter().position(|&r| r == card_name) {
                Some(position) => Ok(Card {
                    name: ROYALTY_CARDS[position],
                    value: 10u8,
                }),
                None => Err(()),
            },
        }
    }
}
impl Default for Card {
    fn default() -> Self {
        Self {
            name: "ERR",
            value: 0,
        }
    }
}

#[cfg(test)]
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.value)
    }
}
