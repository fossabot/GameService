use regex::Regex;
use std::char::ParseCharError;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Standard Card Face (BlackJack)
#[derive(Clone, Debug, Copy)]
pub enum StandardCardFace {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    King,
    Queen,
    Joker,
}

impl Display for StandardCardFace {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::StandardCardFace::*;
        let text_value = match *self {
            Ace => "ACE",
            Two => "TWO",
            Three => "THREE",
            Four => "FOUR",
            Five => "FIVE",
            Six => "SIX",
            Seven => "SEVEN",
            Eight => "Eight",
            Nine => "NINE",
            Ten => "TEN",
            Jack => "JACK",
            King => "KING",
            Queen => "QUEEN",
            Joker => "JOKER",
        };
        write!(f, "{}", text_value)
    }
}

/// Standard Card (BlackJack)
#[derive(Debug, Clone, Copy)]
pub enum StandardCard {
    Hearts(StandardCardFace),
    Spades(StandardCardFace),
    Clubs(StandardCardFace),
    Diamonds(StandardCardFace),
}

impl Display for StandardCard {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::StandardCard::*;
        write!(
            f,
            "{}",
            match *self {
                Hearts(ref inner) => format!("HEARTS:{}", inner.to_string()),
                Spades(ref inner) => format!("SPADES:{}", inner.to_string()),
                Clubs(ref inner) => format!("CLUBS:{}", inner.to_string()),
                Diamonds(ref inner) => format!("DIAMONDS:{}", inner.to_string()),
            }
        )
    }
}

impl FromStr for StandardCard {
    type Err = StandardCardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let txt = s.to_uppercase();
        use self::StandardCard::*;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*):(.*)").unwrap();
        }

        let data = RE.captures_iter(&txt)
            .next()
            .ok_or_else(|| StandardCardParseError::NoCaptureGroup)?;

        let face = match &data[2] {
            "ACE" => StandardCardFace::Ace,
            "TWO" => StandardCardFace::Two,
            "THREE" => StandardCardFace::Three,
            "FOUR" => StandardCardFace::Four,
            "FIVE" => StandardCardFace::Five,
            "SIX" => StandardCardFace::Six,
            "SEVEN" => StandardCardFace::Seven,
            "EIGHT" => StandardCardFace::Eight,
            "NINE" => StandardCardFace::Nine,
            "TEN" => StandardCardFace::Ten,
            "JACK" => StandardCardFace::Jack,
            "KING" => StandardCardFace::King,
            "QUEEN" => StandardCardFace::Queen,
            "JOKER" => StandardCardFace::Joker,
            _ => return Err(StandardCardParseError::InvalidCard),
        };
        Ok(match &data[1] {
            "HEARTS" => Hearts(face),
            "SPADES" => Spades(face),
            "CLUBS" => Clubs(face),
            "DIAMONDS" => Diamonds(face),
            _ => return Err(StandardCardParseError::NoSymbol),
        })
    }
}

#[derive(Clone, Debug)]
pub enum StandardCardParseError {
    InvalidCard,
    NoCaptureGroup,
    NoSymbol,
    ParseChar(ParseCharError),
}

impl Display for StandardCardParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for StandardCardParseError {
    fn description(&self) -> &str {
        use self::StandardCardParseError::*;

        match *self {
            InvalidCard => "Invalid card given",
            NoCaptureGroup => "No regex capture group matched",
            NoSymbol => "No matching symbol found",
            ParseChar(ref inner) => inner.description(),
        }
    }
}
impl StandardCard {
    pub fn face(&self) -> StandardCardFace {
        self.clone().into()
    }
    pub fn suite_string(&self) -> String {
        use self::StandardCard::*;
        String::from(match *self {
            Hearts(_) => "Hearts",
            Clubs(_) => "Clubs",
            Spades(_) => "Spades",
            Diamonds(_) => "Diamonds",
        })
    }
    pub fn face_as_string(&self) -> String {
        self.face().to_string()
    }
}

impl StandardCardFace {
    /// Returns the numerical value of a Face by blackjack standards
    /// Please note, Ace can be both 11 and 1 but is 11 in this instance
    pub fn value(&self) -> u8 {
        self.clone().into()
    }
}

impl From<StandardCard> for StandardCardFace {
    fn from(card: StandardCard) -> StandardCardFace {
        use self::StandardCard::*;
        match card {
            Hearts(inner) | Spades(inner) | Clubs(inner) | Diamonds(inner) => inner,
        }
    }
}

impl From<StandardCardFace> for u8 {
    fn from(face: StandardCardFace) -> u8 {
        use self::StandardCardFace::*;
        match face {
            Ace => 11,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            _ => 10,
        }
    }
}

impl From<StandardCard> for u8 {
    fn from(card: StandardCard) -> u8 {
        u8::from(StandardCardFace::from(card))
    }
}

impl From<StandardCard> for u64 {
    fn from(card: StandardCard) -> u64 {
        u64::from(u8::from(card))
    }
}

/// A Standard Deck Of Cards
pub const STANDARD_DECK_OF_CARDS: [StandardCard; 56] = [
    StandardCard::Hearts(StandardCardFace::Ace),
    StandardCard::Hearts(StandardCardFace::Two),
    StandardCard::Hearts(StandardCardFace::Three),
    StandardCard::Hearts(StandardCardFace::Four),
    StandardCard::Hearts(StandardCardFace::Five),
    StandardCard::Hearts(StandardCardFace::Six),
    StandardCard::Hearts(StandardCardFace::Seven),
    StandardCard::Hearts(StandardCardFace::Eight),
    StandardCard::Hearts(StandardCardFace::Nine),
    StandardCard::Hearts(StandardCardFace::Ten),
    StandardCard::Hearts(StandardCardFace::Jack),
    StandardCard::Hearts(StandardCardFace::Queen),
    StandardCard::Hearts(StandardCardFace::King),
    StandardCard::Hearts(StandardCardFace::Joker),
    StandardCard::Spades(StandardCardFace::Ace),
    StandardCard::Spades(StandardCardFace::Two),
    StandardCard::Spades(StandardCardFace::Three),
    StandardCard::Spades(StandardCardFace::Four),
    StandardCard::Spades(StandardCardFace::Five),
    StandardCard::Spades(StandardCardFace::Six),
    StandardCard::Spades(StandardCardFace::Seven),
    StandardCard::Spades(StandardCardFace::Eight),
    StandardCard::Spades(StandardCardFace::Nine),
    StandardCard::Spades(StandardCardFace::Ten),
    StandardCard::Spades(StandardCardFace::Jack),
    StandardCard::Spades(StandardCardFace::Queen),
    StandardCard::Spades(StandardCardFace::King),
    StandardCard::Spades(StandardCardFace::Joker),
    StandardCard::Clubs(StandardCardFace::Ace),
    StandardCard::Clubs(StandardCardFace::Two),
    StandardCard::Clubs(StandardCardFace::Three),
    StandardCard::Clubs(StandardCardFace::Four),
    StandardCard::Clubs(StandardCardFace::Five),
    StandardCard::Clubs(StandardCardFace::Six),
    StandardCard::Clubs(StandardCardFace::Seven),
    StandardCard::Clubs(StandardCardFace::Eight),
    StandardCard::Clubs(StandardCardFace::Nine),
    StandardCard::Clubs(StandardCardFace::Ten),
    StandardCard::Clubs(StandardCardFace::Jack),
    StandardCard::Clubs(StandardCardFace::Queen),
    StandardCard::Clubs(StandardCardFace::King),
    StandardCard::Clubs(StandardCardFace::Joker),
    StandardCard::Diamonds(StandardCardFace::Ace),
    StandardCard::Diamonds(StandardCardFace::Two),
    StandardCard::Diamonds(StandardCardFace::Three),
    StandardCard::Diamonds(StandardCardFace::Four),
    StandardCard::Diamonds(StandardCardFace::Five),
    StandardCard::Diamonds(StandardCardFace::Six),
    StandardCard::Diamonds(StandardCardFace::Seven),
    StandardCard::Diamonds(StandardCardFace::Eight),
    StandardCard::Diamonds(StandardCardFace::Nine),
    StandardCard::Diamonds(StandardCardFace::Ten),
    StandardCard::Diamonds(StandardCardFace::Jack),
    StandardCard::Diamonds(StandardCardFace::Queen),
    StandardCard::Diamonds(StandardCardFace::King),
    StandardCard::Diamonds(StandardCardFace::Joker),
];

#[cfg(test)]
mod test {
    use super::{StandardCard, STANDARD_DECK_OF_CARDS};
    const CARDS: [&str; 56] = [
        "HEARTS:ACE",
        "HEARTS:TWO",
        "HEARTS:THREE",
        "HEARTS:FOUR",
        "HEARTS:FIVE",
        "HEARTS:SIX",
        "HEARTS:SEVEN",
        "HEARTS:EIGHT",
        "HEARTS:NINE",
        "HEARTS:TEN",
        "HEARTS:JACK",
        "HEARTS:KING",
        "HEARTS:QUEEN",
        "HEARTS:JOKER",
        "CLUBS:ACE",
        "CLUBS:TWO",
        "CLUBS:THREE",
        "CLUBS:FOUR",
        "CLUBS:FIVE",
        "CLUBS:SIX",
        "CLUBS:SEVEN",
        "CLUBS:EIGHT",
        "CLUBS:NINE",
        "CLUBS:TEN",
        "CLUBS:JACK",
        "CLUBS:KING",
        "CLUBS:QUEEN",
        "CLUBS:JOKER",
        "SPADES:ACE",
        "SPADES:TWO",
        "SPADES:THREE",
        "SPADES:FOUR",
        "SPADES:FIVE",
        "SPADES:SIX",
        "SPADES:SEVEN",
        "SPADES:EIGHT",
        "SPADES:NINE",
        "SPADES:TEN",
        "SPADES:JACK",
        "SPADES:KING",
        "SPADES:QUEEN",
        "SPADES:JOKER",
        "DIAMONDS:ACE",
        "DIAMONDS:TWO",
        "DIAMONDS:THREE",
        "DIAMONDS:FOUR",
        "DIAMONDS:FIVE",
        "DIAMONDS:SIX",
        "DIAMONDS:SEVEN",
        "DIAMONDS:EIGHT",
        "DIAMONDS:NINE",
        "DIAMONDS:TEN",
        "DIAMONDS:JACK",
        "DIAMONDS:KING",
        "DIAMONDS:QUEEN",
        "DIAMONDS:JOKER",
    ];

    #[test]
    fn deck_of_cards() {
        assert_eq!(STANDARD_DECK_OF_CARDS.len(), 56);
        for card in CARDS.iter() {
            card.to_string()
                .parse::<StandardCard>()
                .expect(&format!("Failed to parse {}", card));
        }
        for card in STANDARD_DECK_OF_CARDS.iter() {
            card.to_string()
                .parse::<StandardCard>()
                .expect(&format!("Failed to parse {}", card));
        }
    }
}
