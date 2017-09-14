#[cfg(test)]
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub value: u8,
}

impl Card {
    // Used to restore from DB
    pub fn new(card_name: &str) -> Option<Self> {
        let pos = vec![
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
        ].iter()
            .position(|&r| r == card_name);
        match pos {
            Some(position) => Some(Card {
                name: card_name.to_string(),
                value: position as u8 + 1u8,
            }),
            None => {
                if vec!["JACKS", "KINGS", "QUEENS"].iter().any(
                    |&r| r == card_name,
                )
                {
                    Some(Card {
                        name: card_name.to_string(),
                        value: 10u8,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.value)
    }
}
