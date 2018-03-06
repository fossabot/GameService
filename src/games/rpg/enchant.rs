use super::Gear;
use super::errors::EnchantError;
pub struct Enchanter;
use rand::{self, Rng};
use std::i64;

impl Enchanter {
    /// Returns the ods of enchantment
    pub fn odds(enchantment_lvl: u8) -> Result<f64, EnchantError> {
        if enchantment_lvl == 255 {
            Err(EnchantError::MaxEnchant)
        } else {
            Ok(f64::from(enchantment_lvl + 1).recip())
        }
    }

    /// Attempts to encrease gear enchantment lvl, failure results in reduction
    pub fn enchant_gear(g: &mut Gear) -> Result<Gear, EnchantError> {
        if g.enchant == 255 {
            Err(EnchantError::MaxEnchant)
        } else {
            let mut gear = g.clone();
            if rand::thread_rng().gen_weighted_bool((u32::from(gear.enchant) + 1) * 2) {
                gear.increase_enchant(1)
            } else {
                gear.decrease_enchant(1)
            };
            Ok(gear)
        }
    }

    /// Rerol the lowest stat of the current gear within a certain range
    pub fn reroll_stats(g: &mut Gear) -> Result<Gear, EnchantError> {
        if g.enchant < 15 {
            return Err(EnchantError::NotHighEnoughEnchantment(15));
        }
        let min = (String::new(), i64::MIN);
        let mut rng = rand::thread_rng();
        // Get the lowest stat
        let mut stat = [
            (String::from("acc"), g.accuracy),
            (String::from("atk"), g.attack),
            (String::from("def"), g.defense),
            (String::from("eva"), g.evasion),
            (String::from("hp"), g.health),
        ].into_iter()
            .fold(&min, |high, low| {
                if (low.1 > 0) & (high.1 < low.1) {
                    low
                } else {
                    high
                }
            })
            .clone();
        let mut gear = g.clone();
        gear.decrease_enchant(15);
        stat.1 = rng.gen_range(
            stat.1.checked_sub(-5).unwrap_or(i64::MIN),
            stat.1.checked_add(10).unwrap_or(i64::MAX),
        );
        match stat.0.as_str() {
            "acc" => gear.accuracy = stat.1,
            "atk" => gear.attack = stat.1,
            "def" => gear.defense = stat.1,
            "eva" => gear.evasion = stat.1,
            "hp" => gear.health = stat.1,
            _ => unreachable!(),
        }
        Ok(gear)
    }

    /// Curses a random positive stat and gives a certain % of that value to another positive stat
    pub fn curse_gear(g: &mut Gear) -> Result<Gear, EnchantError> {
        if g.enchant < 18 {
            return Err(EnchantError::NotHighEnoughEnchantment(18));
        }
        let stats: Vec<(String, i64)> = vec![
            (String::from("acc"), g.accuracy),
            (String::from("atk"), g.attack),
            (String::from("def"), g.defense),
            (String::from("eva"), g.evasion),
            (String::from("hp"), g.health),
        ].into_iter()
            .filter(|stat| stat.1 <= 0)
            .collect();
        if stats.len() < 4 {
            return Err(EnchantError::MaxCurses);
        }
        let mut rng = rand::thread_rng();
        let &(ref name1, mut stat1) = rng.choose(&stats).unwrap();
        let &(ref name2, mut stat2) = rng.choose(&stats).unwrap();
        stat1 = -(stat1 as f64 / 4.0).round() as i64;
        stat2 -= stat1;

        let mut gear = g.clone();
        gear.decrease_enchant(18);
        for stat in &[(name1, stat1), (name2, stat2)] {
            match stat.0.as_str() {
                "acc" => gear.accuracy = stat.1,
                "atk" => gear.attack = stat.1,
                "def" => gear.defense = stat.1,
                "eva" => gear.evasion = stat.1,
                "hp" => gear.health = stat.1,
                _ => unreachable!(),
            }
        }
        Ok(gear)
    }
    // Implement Transmute
    // Transmute will convert gear to a boss item
    // or regress its base stats
}
