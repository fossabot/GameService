// TODO: Return only Result<T, E> use a transformer to change response

use rand::{thread_rng, Rng};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const WEIGHT: u32 = 2;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Coin {
    Heads,
    Tails,
}

#[derive(Debug)]
pub struct CoinParseError {
    description: String,
    origional_str: String,
}

impl CoinParseError {
    pub fn new(s: &str) -> CoinParseError {
        CoinParseError {
            description: format!("Failed to parse coin from `{}`", s),
            origional_str: s.to_string(),
        }
    }
}

impl Error for CoinParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for CoinParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl FromStr for Coin {
    type Err = CoinParseError;
    fn from_str(s: &str) -> Result<Coin, CoinParseError> {
        if (s.is_empty() || s.len() > 5) || !s.to_lowercase().chars().enumerate().all(|(i, c)| {
            match i {
                0 => c == 'h' || c == 't',
                1 => c == 'e' || c == 'a',
                2 => c == 'a' || c == 'i',
                3 => c == 'd' || c == 'l',
                4 => c == 's',
                _ => false,
            }
        }) {
            Err(CoinParseError::new(s)) // Parse Error
        } else if s.starts_with('h') {
            Ok(Coin::Heads)
        } else {
            Ok(Coin::Tails)
        }
    }
}

impl Coin {
    /// Flips a coun, returning a Coin with its current Side up
    pub fn flip() -> Coin {
        let mut rng = thread_rng();
        if rng.gen_weighted_bool(WEIGHT) {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinTossResult {
    pub player: Coin,
    pub computer: Coin,
    pub bet: u64,
    pub gain: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinTossResponse {
    status_code: u16,
    status: Result<CoinTossResult, String>,
}

impl CoinTossResponse {
    pub fn win(bet: u64, side: Coin, guess: Coin) -> Self {
        Self {
            status_code: 200,
            status: Ok(CoinTossResult {
                player: guess,
                computer: side,
                bet,
                gain: (bet / 2) as i64,
            }),
        }
    }

    pub fn lose(bet: u64, side: Coin, guess: Coin) -> Self {
        Self {
            status_code: 200,
            status: Ok(CoinTossResult {
                player: guess,
                computer: side,
                bet,
                gain: -(bet as i64),
            }),
        }
    }

    pub fn err(err: String) -> Self {
        Self {
            status_code: 501,
            status: Err(err),
        }
    }
    pub fn result(&self) -> &Result<CoinTossResult, String> {
        &self.status
    }
}

/// Guess a coin side
pub fn guess_side(bet: u64, side: &str) -> CoinTossResponse {
    let side_lowercase = side.to_lowercase();

    if !["heads", "h", "tails", "t"].contains(&&side_lowercase[..]) {
        return CoinTossResponse::err(String::from("Not a valid side, heads/tails."));
    };

    let guessed_side = if side.starts_with('h') {
        Coin::Heads
    } else {
        Coin::Tails
    };

    let side = Coin::flip();

    if guessed_side == side {
        CoinTossResponse::win(bet, side, guessed_side)
    } else {
        CoinTossResponse::lose(bet, side, guessed_side)
    }
}

#[cfg(test)]
mod test {
    use super::Coin;

    fn parse_coin_or_panic(s: &str) -> Coin {
        match s.parse::<Coin>() {
            Ok(c) => c,
            Err(why) => panic!("{}", why),
        }
    }
    #[test]
    fn parse_coin() {
        parse_coin_or_panic("h");
        parse_coin_or_panic("t");
        parse_coin_or_panic("H");
        parse_coin_or_panic("T");
        parse_coin_or_panic("he");
        parse_coin_or_panic("hea");
        parse_coin_or_panic("head");
        parse_coin_or_panic("heads");
        parse_coin_or_panic("ta");
        parse_coin_or_panic("tai");
        parse_coin_or_panic("tail");
        parse_coin_or_panic("tails");
        if let Ok(coin) = "Heading".parse::<Coin>() {
            panic!(
                "Should `Heading` == `{:?}`? No? Didn't think so, fix it.",
                coin
            )
        }
        if let Ok(coin) = "Tacito".parse::<Coin>() {
            panic!("Should `Tacito` really equal `{:?}`", coin)
        }
    }

}
