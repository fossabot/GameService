mod cards;
mod deck;
mod deck_of_cards;

pub mod blackjack;

pub mod coin_toss;
pub mod rpg;
pub mod rps;
pub mod slot_machine;
// pub mod gofish;

pub use self::cards::{
    StandardCard, StandardCardFace, StandardCardParseError, STANDARD_DECK_OF_CARDS,
};
pub use self::deck::{StandardDeck, StandardDeckError};
pub use self::deck_of_cards::STANDARD_DECK_OF_CARDS as STANDARD_BJ_DECK_OF_CARDS;
