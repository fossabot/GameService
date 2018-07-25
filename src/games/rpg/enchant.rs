use super::errors::EnchantError;
use super::Gear;
pub struct Enchanter;
use rand::{self, Rng};

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
        let enchant = gear.enchant_lvl()?;
        let max = gear.clone().max_enchant()?;
        if enchant == max {
            Err(EnchantError::MaxEnchant)
        } else if rand::thread_rng().gen_bool(1f64 / (enchant as f64 + 1_f64) * 2_f64) {
            gear.increase_enchant(1)?;
            Ok(true)
        } else {
            gear.decrease_enchant(1)?;
            Ok(false)
        }
    }

    // /// Rerol the lowest stat of the current gear within a certain range
    // pub fn reroll_stats(gear: &mut Gear) -> Result<(), EnchantError> {.. }
    // Removed

    //     /// Curses a random positive stat and gives a certain % of that value to another positive stat
    //     pub fn curse_gear<T>(gear: &mut T) -> Result<(), EnchantError> {
    //         if gear.enchant < 18 {
    //             return Err(EnchantError::NotHighEnoughEnchantment{ req:18});
    //         }
    //         let stats: Vec<(String, i64)> = vec![
    //             (String::from("acc"), gear.accuracy),
    //             (String::from("atk"), gear.attack),
    //             (String::from("def"), gear.defense),
    //             (String::from("eva"), gear.evasion),
    //             (String::from("hp"), gear.health),
    //         ].into_iter()
    //             .filter(|stat| stat.1 >= 0)
    //             .collect();
    //         if stats.len() < 4 {
    //             return Err(EnchantError::MaxCurses);
    //         }
    //         let mut rng = rand::thread_rng();
    //         let &(ref name1, mut stat1) = rng.choose(&stats).unwrap();
    //         let &(ref name2, mut stat2) = rng.choose(&stats).unwrap();
    //         stat1 = -(stat1 as f64 / 4.0).round() as i64;
    //         stat2 -= stat1;
    //         for stat in &[(name1, stat1), (name2, stat2)] {
    //             match stat.0.as_str() {
    //                 "acc" => gear.accuracy = stat.1,
    //                 "atk" => gear.attack = stat.1,
    //                 "def" => gear.defense = stat.1,
    //                 "eva" => gear.evasion = stat.1,
    //                 "hp" => gear.health = stat.1,
    //                 _ => unreachable!(),
    //             }
    //         }
    //         gear.decrease_enchant(18);
    //         Ok(())
    //    }
}

#[cfg(test)]
mod test {
    use super::super::gear::{Gear, GearID, GearInfoStore};
    use super::Enchanter;
    use games::rpg::errors;
    // Returns test gear
    fn test_gear(id: GearID, enchant: u8, divinity: u16) -> Gear {
        let g: Result<Gear, errors::GearParseError> =
            GearInfoStore::new(id, enchant, divinity).into();
        g.unwrap()
    }

    #[test]
    fn enchant() {
        let mut gear = test_gear(0, 1, 0);
        let success = Enchanter::enchant_gear(&mut gear).unwrap();
        if success {
            assert_eq!(
                gear.enchant_lvl()
                    .expect("Gear wasnt enchantable, pick another ID"),
                2
            )
        } else {
            assert_eq!(
                gear.enchant_lvl()
                    .expect("Gear wasnt enchantable, pick another ID"),
                0
            )
        }
        let max = gear.clone().max_enchant().expect("Gear is not enchantable");

        gear.increase_enchant(max)
            .expect("Expected an error but it didnt occur");
        Enchanter::enchant_gear(&mut gear).expect_err("Expected error but it succeed");
    }

    // #[test]
    // fn curse() {}

    // #[test]
    // fn reroll() {}
}
