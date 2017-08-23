#[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Card {
    pub name: String,
    pub value: u16
}
impl Card {
    pub fn new(name: &str, value: u16) -> Self {
        return Card {name: name.to_owned(), value: value};
    }
}



pub fn create_deck() -> Vec<Card> {
    vec![        
        Card::new("ACE", 11),
        Card::new("TWOS", 2),
        Card::new("THREES", 3),
        Card::new("FOURS", 4),
        Card::new("FIVES", 5),
        Card::new("SIXES", 6),
        Card::new("SEVENS", 7),
        Card::new("EIGHTS", 8),
        Card::new("NINES", 9),
        Card::new("TENS", 10),
        Card::new("JACKS", 10),
        Card::new("KINGS", 10),
        Card::new("QUEENS", 10),
        Card::new("ACE", 11),
        Card::new("TWOS", 2),
        Card::new("THREES", 3),
        Card::new("FOURS", 4),
        Card::new("FIVES", 5),
        Card::new("SIXES", 6),
        Card::new("SEVENS", 7),
        Card::new("EIGHTS", 8),
        Card::new("NINES", 9),
        Card::new("TENS", 10),
        Card::new("JACKS", 10),
        Card::new("KINGS", 10),
        Card::new("QUEENS", 10),
        Card::new("ACE", 11),
        Card::new("TWOS", 2),
        Card::new("THREES", 3),
        Card::new("FOURS", 4),
        Card::new("FIVES", 5),
        Card::new("SIXES", 6),
        Card::new("SEVENS", 7),
        Card::new("EIGHTS", 8),
        Card::new("NINES", 9),
        Card::new("TENS", 10),
        Card::new("JACKS", 10),
        Card::new("KINGS", 10),
        Card::new("QUEENS", 10),
        Card::new("ACE", 11),
        Card::new("TWOS", 2),
        Card::new("THREES", 3),
        Card::new("FOURS", 4),
        Card::new("FIVES", 5),
        Card::new("SIXES", 6),
        Card::new("SEVENS", 7),
        Card::new("EIGHTS", 8),
        Card::new("NINES", 9),
        Card::new("TENS", 10),
        Card::new("JACKS", 10),
        Card::new("KINGS", 10),
        Card::new("QUEENS", 10),
    ]
}
