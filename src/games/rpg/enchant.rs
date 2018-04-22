use super::errors::EnchantError;
use super::Gear;
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
    pub fn enchant_gear(gear: &mut Gear) -> Result<bool, EnchantError> {
        if gear.enchant == 255 {
            Err(EnchantError::MaxEnchant)
        } else if rand::thread_rng().gen_weighted_bool((u32::from(gear.enchant) + 1) * 2) {
            gear.increase_enchant(1);
            Ok(true)
        } else {
            gear.decrease_enchant(1);
            Ok(false)
        }
    }

    /// Rerol the lowest stat of the current gear within a certain range
    pub fn reroll_stats(gear: &mut Gear) -> Result<(), EnchantError> {
        if gear.enchant < 15 {
            return Err(EnchantError::NotHighEnoughEnchantment(15));
        }
        let min = (String::new(), i64::MIN);
        let mut rng = rand::thread_rng();
        // Get the lowest stat
        let mut stat = [
            (String::from("acc"), gear.accuracy),
            (String::from("atk"), gear.attack),
            (String::from("def"), gear.defense),
            (String::from("eva"), gear.evasion),
            (String::from("hp"), gear.health),
        ].into_iter()
            .fold(&min, |high, low| {
                if (low.1 > 0) & (high.1 < low.1) {
                    low
                } else {
                    high
                }
            })
            .clone();
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
        Ok(())
    }

    /// Curses a random positive stat and gives a certain % of that value to another positive stat
    pub fn curse_gear(gear: &mut Gear) -> Result<(), EnchantError> {
        if gear.enchant < 18 {
            return Err(EnchantError::NotHighEnoughEnchantment(18));
        }
        let stats: Vec<(String, i64)> = vec![
            (String::from("acc"), gear.accuracy),
            (String::from("atk"), gear.attack),
            (String::from("def"), gear.defense),
            (String::from("eva"), gear.evasion),
            (String::from("hp"), gear.health),
        ].into_iter()
            .filter(|stat| stat.1 >= 0)
            .collect();
        if stats.len() < 4 {
            return Err(EnchantError::MaxCurses);
        }
        let mut rng = rand::thread_rng();
        let &(ref name1, mut stat1) = rng.choose(&stats).unwrap();
        let &(ref name2, mut stat2) = rng.choose(&stats).unwrap();
        stat1 = -(stat1 as f64 / 4.0).round() as i64;
        stat2 -= stat1;
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
        gear.decrease_enchant(18);
        Ok(())
    }
    // Implement Transmute
    // Transmute will convert gear to a boss item
    // or regress its base stats
}

#[cfg(test)]
mod test {
    use super::super::gear::{Gear, GearType};
    use super::{EnchantError, Enchanter};
    // Returns test gear
    fn test_gear() -> Gear {
        Gear {
            attack: 1,
            accuracy: 1,
            defense: 1,
            enchant: 1,
            evasion: 1,
            gear_type: GearType::Weapon,
            health: 1,
            is_boss: false,
            name: String::from("Stick of justice"),
        }
    }
    #[test]
    fn enchant() {
        let mut gear = test_gear();
        let success = Enchanter::enchant_gear(&mut gear).unwrap();
        if success {
            assert_eq!(gear.enchant, 2)
        } else {
            assert_eq!(gear.enchant, 0)
        }
        gear.enchant = 255;
        Enchanter::enchant_gear(&mut gear).expect_err("Expected enchant error, but it didnt occur");
    }

    #[test]
    fn curse() {
        let mut gear = test_gear();
        gear.enchant = 17;
        match Enchanter::curse_gear(&mut gear) {
            Err(err) => match err {
                EnchantError::NotHighEnoughEnchantment(_) => (),
                _ => panic!(
                    "Expected Error \"Not high Enough Enchantment\" Found, {}",
                    err
                ),
            },
            _ => panic!("Expected Error \"Not High Enough Enchantment\" but successfully cursed"),
        }
        assert_eq!(gear.enchant, 17);
        gear.enchant = 100;
        match Enchanter::curse_gear(&mut gear) {
            Ok(_) => (), // TODO: Write a test for curse stats
            Err(why) => panic!("{}", why),
        }
        assert_eq!(gear.enchant, 82);
    }

    #[test]
    fn reroll() {
        let mut gear = test_gear();
        gear.accuracy = 20;
        gear.enchant = 14;
        match Enchanter::reroll_stats(&mut gear) {
            Err(err) => match err {
                EnchantError::NotHighEnoughEnchantment(_) => (),
                _ => panic!(
                    "Expected Error \"Not high Enough Enchantment\" Found, {}",
                    err
                ),
            },
            _ => panic!("Expected Error \"Not High Enough Enchantment\" but successfully rerolled"),
        }
        assert_eq!(gear.enchant, 14);
        gear.enchant = 15;
        let pre_acc = gear.accuracy();
        match Enchanter::reroll_stats(&mut gear) {
            Ok(_) => assert!(pre_acc != gear.accuracy()),
            Err(why) => panic!("{}", why),
        }
        assert_eq!(gear.enchant, 0)
    }
}
