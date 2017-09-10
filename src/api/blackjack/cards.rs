use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub value: u8,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.value)
    }
}
