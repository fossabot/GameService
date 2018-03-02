pub use super::super::{StandardDeckError as DeckError, StandardCardParseError as CardParseError};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug)]
pub enum GoFishError {
	NoCard,
	CardParseError
}

impl Display for GoFishError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for GoFishError {
	fn description(&self) -> &str {
		use self::GoFishError::*;
		match *self {
			NoCard => "No cards remaining in deck",
			CardParseError => "Failed to parse Card"

		}
	}
}

impl From<DeckError> for GoFishError {
	fn from(err: DeckError ) -> GoFishError {
		GoFishError::NoCard
	}
}

impl From<CardParseError> for GoFishError {
	fn from(err: CardParseError) -> GoFishError {
		GoFishError::CardParseError
	}
}