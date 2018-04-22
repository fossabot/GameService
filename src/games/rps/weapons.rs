use rand::{thread_rng, Rng};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Rock Paper Scissors
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Weapons {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug)]
pub struct WeaponParseError {
    description: String,
}

impl fmt::Display for WeaponParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Valid options are Rock/Paper/Scissors")
    }
}

impl Error for WeaponParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Weapons {
    pub fn to_string(&self) -> String {
        match *self {
            Weapons::Rock => String::from("Rock"),
            Weapons::Paper => String::from("Paper"),
            Weapons::Scissors => String::from("Scissors"),
        }
    }

    /// Returns a random Weapon
    pub fn rand_weapon() -> Self {
        // Although the type system designates that selecting an Nth char of a
        // string can result in None, as N is always 0 through 2 it won't be.
        //
        // Regardless, this match statement covers that non-existent case.
        match "rps".chars().nth(thread_rng().gen_range(0, 3)) {
            Some('p') => Weapons::Paper,
            Some('s') => Weapons::Scissors,
            _ => Weapons::Rock,
        }
    }
}

impl FromStr for Weapons {
    type Err = WeaponParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err {
                description: String::from("... you have no weapon ... "),
            });
        }
        if !s.to_lowercase().chars().enumerate().all(|(i, c)| match i {
            0 => c == 'r' || c == 'p' || c == 's',
            1 => c == 'o' || c == 'a' || c == 'c',
            2 => c == 'c' || c == 'p' || c == 'i',
            3 => c == 'k' || c == 'e' || c == 's',
            4 => c == 'r' || c == 's',
            5 => c == 'o',
            6 => c == 'r',
            7 => c == 's',
            _ => false,
        }) {
            Err(Self::Err {
                description: format!("{} is not a valid weapon", s.to_string()),
            })
        } else {
            match s.to_lowercase().chars().nth(0) {
                Some(wep) => Ok(match wep {
                    'r' => Weapons::Rock,
                    'p' => Weapons::Paper,
                    's' => Weapons::Scissors,
                    _ => {
                        return Err(Self::Err {
                            description: s.to_string(),
                        })
                    }
                }),
                None => Err(Self::Err {
                    description: s.to_string(),
                }),
            }
        }
    }
}

impl PartialOrd for Weapons {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Makes creating the game much simpler
impl Ord for Weapons {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Weapons::Rock => match *other {
                Weapons::Rock => Ordering::Equal,
                Weapons::Paper => Ordering::Less,
                Weapons::Scissors => Ordering::Greater,
            },
            Weapons::Paper => match *other {
                Weapons::Rock => Ordering::Greater,
                Weapons::Paper => Ordering::Equal,
                Weapons::Scissors => Ordering::Less,
            },
            Weapons::Scissors => match *other {
                Weapons::Rock => Ordering::Less,
                Weapons::Paper => Ordering::Greater,
                Weapons::Scissors => Ordering::Equal,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Weapons;
    #[test]
    fn comparision() {
        assert_eq!(Weapons::Rock, Weapons::Rock);
        assert!(Weapons::Rock > Weapons::Scissors);
        assert!(Weapons::Rock < Weapons::Paper);
        assert_eq!(Weapons::Paper, Weapons::Paper);
        assert!(Weapons::Paper > Weapons::Rock);
        assert!(Weapons::Paper < Weapons::Scissors);
        assert_eq!(Weapons::Scissors, Weapons::Scissors);
        assert!(Weapons::Scissors > Weapons::Paper);
        assert!(Weapons::Scissors < Weapons::Rock);
    }

    fn parse_wep_or_panic(s: &str) {
        match s.parse::<Weapons>() {
            Ok(_) => (),
            Err(why) => panic!("{}", why),
        }
    }

    #[test]
    fn from_string() {
        parse_wep_or_panic("R");
        parse_wep_or_panic("ROCK");
        parse_wep_or_panic("rock");
        parse_wep_or_panic("S");
        parse_wep_or_panic("SCIS");
        parse_wep_or_panic("scissors");
        parse_wep_or_panic("P");
        parse_wep_or_panic("PAPE");
        parse_wep_or_panic("paper");
        if let Ok(wep) = "ROCKET".parse::<Weapons>() {
            panic!(
                "Successfully parsed \"ROCKET\" as {:?}! Hmm... that doesn't seem right.",
                wep
            )
        }
        if let Ok(wep) = "SCOOTER".parse::<Weapons>() {
            panic!(
                "Successfully parsed \"SCOOTER\" Parsed as {:?}! Hmm... that doesnt seem right.",
                wep
            )
        }
        if let Ok(wep) = "PAPI".parse::<Weapons>() {
            panic!(
                "Successfully parsed \"PAPI\" Parsed as {:?}! Hmm... that doesnt seem right",
                wep
            )
        }
    }
}
