use super::Card;
use super::GoFishError;

pub struct Hand {
	pub hand: Vec<Card>
}


impl Hand {
	pub fn check(&self, value: Card) -> bool {
		self.hand.iter().any(|c| c.to_string() == value.to_string())
	}
}