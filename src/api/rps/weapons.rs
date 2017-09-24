use std::cmp::{PartialOrd, Ord, Ordering};
use rand::{thread_rng, Rng};
/// Rock Paper Scissors
#[derive(PartialEq, Eq)]
pub enum Weapons {
    Rock,
    Paper,
    Scissors,
}

impl Weapons {
    pub fn from_str(weapon: &str) -> Option<Self>{
        match weapon.to_lowercase().chars().nth(0) {
            Some(wep) => match wep {
                'r' => Some(Weapons::Rock),
                'p' => Some(Weapons::Paper),
                's' => Some(Weapons::Scissors),
                _ => None
            },
            None => None
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Weapons::Rock => String::from("Rock"),
            Weapons::Paper => String::from("Paper"),
            Weapons::Scissors => String::from("Scissors")
        }
    }
    pub fn rand_weapon() -> Self {
        match "rps".chars().nth(thread_rng().gen_range(0, 3)).unwrap() {
                'r' => Weapons::Rock,
                'p' => Weapons::Paper,
                's' => Weapons::Scissors,
                _ => Weapons::Rock,
        }
    }

}
impl PartialOrd for Weapons {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Weapons {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            &Weapons::Rock => {
                match other {
                    &Weapons::Rock => Ordering::Equal,
                    &Weapons::Paper => Ordering::Less,
                    &Weapons::Scissors => Ordering::Greater,
                }
            },
            &Weapons::Paper => {
                match other {
                    &Weapons::Rock => Ordering::Greater,
                    &Weapons::Paper => Ordering::Equal,
                    &Weapons::Scissors => Ordering::Less,
                }
            },
            &Weapons::Scissors => {
                match other {
                    &Weapons::Rock => Ordering::Less,
                    &Weapons::Paper => Ordering::Greater,
                    &Weapons::Scissors => Ordering::Equal
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Weapons;
    #[test]
    fn test_comparision() {
        assert!(Weapons::Rock == Weapons::Rock);
        assert!(Weapons::Rock > Weapons::Scissors);
        assert!(Weapons::Rock < Weapons::Paper);
        assert!(Weapons::Paper == Weapons::Paper);
        assert!(Weapons::Paper > Weapons::Rock);
        assert!(Weapons::Paper < Weapons::Scissors);
        assert!(Weapons::Scissors == Weapons::Scissors);
        assert!(Weapons::Scissors > Weapons::Paper);
        assert!(Weapons::Scissors < Weapons::Rock);
    }
}
