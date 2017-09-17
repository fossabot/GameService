#[cfg(test)]
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub value: u8,
}

impl Card {
    // Used to restore from DB
    pub fn new(card_name: &str) -> Result<Self, ()> {
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
            Some(position) => Ok(Card {
                name: card_name.to_string(),
                value: position as u8 + 1u8,
            }),
            None => if vec!["JACKS", "KINGS", "QUEENS"]
                .iter()
                .any(|&r| r == card_name)
            {
                Ok(Card {
                    name: card_name.to_string(),
                    value: 10u8,
                })
            } else {
                Err(())
            },
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            name: "ERR".to_owned(),
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
