extern crate regex;
use self::regex::Regex;
#[cfg(test)]
use std::fmt;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: &'static str,
    pub value: u8,
    pub symbol: &'static str,
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

const VALID_SYMBOLS: [&'static str; 4] = ["HEARTS", "SPADES", "CLUBS", "DIAMONDS"];

const ROYALTY_CARDS: [&'static str; 3] = ["JACKS", "KINGS", "QUEENS"];

impl Card {
    // Used to restore from DB
    // TODO: convert this to, `from_str`
    pub fn new(card_name: &str) -> Self {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(.*):(.*)").unwrap();
        }
        let data = RE.captures_iter(card_name).next().unwrap();
        let symbol_pos = VALID_SYMBOLS.iter().position(|&r| r == &data[1]).unwrap();

        match NON_ROYALTY_CARDS.iter().position(|&r| r == &data[2]) {
            Some(position) => Card {
                name: NON_ROYALTY_CARDS[position],
                value: position as u8 + 1u8,
                symbol: VALID_SYMBOLS[symbol_pos],
            },
            None => match ROYALTY_CARDS.iter().position(|&r| r == &data[2]) {
                Some(position) => Card {
                    name: ROYALTY_CARDS[position],
                    value: 10u8,
                    symbol: VALID_SYMBOLS[symbol_pos],
                },
                None => unreachable!(), // If this reached, fix your data
            },
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.symbol, self.name)
    }
}
impl Default for Card {
    fn default() -> Self {
        Self {
            name: "ERR",
            value: 0,
            symbol: "ERR",
        }
    }
}

#[cfg(test)]
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{} ({})", self.symbol, self.name, self.value)
    }
}
