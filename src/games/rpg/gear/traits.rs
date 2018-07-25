use super::{errors, GearType};
use std::u8;

const DIVINITY_BOOST: u16 = 1_000;

pub trait Gear {
    const GEAR_TYPE: GearType;
    const NAME: &'static str;
    const ATTACK: i32 = 0;
    const ARMOR: i32 = 0;
    const ACCURACY: i32 = 0;
    const EVASION: i32 = 0;
    const HEALTH: i32 = 0;
    const DESCRIPTION: &'static str;
    const MAX_ENCHANT: Option<u8> = None;
    const GAIN_PER_ENCHANT: Option<f32> = None;
    const MAX_DIVINITY: Option<u16> = None;
    fn attack(&self) -> i32 {
        if (Self::MAX_ENCHANT != 0) & (Self::ATTACK != 0) {
            (self.multiplyer() * Self::ATTACK as f32) as i32 + Self::ATTACK
        } else {
            Self::ATTACK
        }
    }
    fn armor(&self) -> i32 {
        if (Self::MAX_ENCHANT != 0) & (Self::ARMOR != 0) {
            (self.multiplyer() * Self::ARMOR as f32) as i64 + Self::ARMOR
        } else {
            Self::ARMOR
        }
    }
    fn accuracy(&self) -> i32 {
        if (Self::MAX_ENCHANT != 0) & (Self::ACCURACY != 0) {
            (self.multiplyer() * Self::ACCURACY as f32) as i32 + Self::ACCURACY
        } else {
            Self::ACCURACY
        }
    }
    fn evasion(&self) -> i32 {
        if (Self::MAX_ENCHANT != 0) & (Self::EVASION != 0) {
            (self.multiplyer() * Self::EVASION as f32) as i32 + Self::EVASION
        } else {
            Self::EVASION
        }
    }
    fn health(&self) -> i32 {
        if (Self::MAX_ENCHANT != 0) & (Self::HEALTH != 0) {
            (self.multiplyer() * Self::HEALTH as f32) as i32 + Self::HEALTH
        } else {
            Self::HEALTH
        }
    }
    fn enchant_lvl(&self) -> Option<u8> {
        None
    }
    fn divinity(&self) -> Option<u16> {
        None
    }
    fn pretty_text(&self) -> String {
        if Self::MAX_ENCHANT == 0 {
            format!("Name: {}\nType: {}\nAttack: {}\nArmor: {}\nEvasion: {}\nHealth: {}\nDescription:\n{}",
                Self::NAME,
                Self::GEAR_TYPE,
                pretty_num!(self.attack()),
                pretty_num!(self.armor()),
                pretty_num!(self.evasion()),
                pretty_num!(self.health()),
                Self::DESCRIPTION
            )
        } else {
            format!("Name: {}\nType: {}\nAttack: {}\nAccuracy: {}\nArmor: {}\nEvasion: {}\nHealth: {}\nEnchantment: {}/{}\nDivinity: {}/{}\nDescription:\n{}",
                Self::NAME,
                Self::GEAR_TYPE,
                pretty_num!(self.attack()),
                pretty_num!(self.accuracy()),
                pretty_num!(self.defense()),
                pretty_num!(self.evasion()),
                pretty_num!(self.health()),
                pretty_num!(self.enchant_lvl()),
                pretty_num!(Self::MAX_ENCHANT),
                pretty_num!(self.divinity()),
                pretty_num!(Self::MAX_DIVINITY),
                Self::DESCRIPTION
            )
        }
    }

    /// Multiplyer of stats gained by enchanting or
    fn multiplyer(&self) -> f32 {
        match self.enchant_lvl() {
            Some(lvl) => {
                (lvl as f32 * Self::GAIN_PER_ENCHANT.unwrap_or_default())
                    + (self.divinity().unwrap_or_default() as f32 / DIVINITY_BOOST as f32)
            }
            None => 0f32,
        }
    }
    fn increase_enchant(&mut self) -> Result<u8, errors::EnchantError> {
        Err(errors::EnchantError::NotEnchantable)
    }
    fn decrease_enchant(&mut self) -> Result<u8, errors::EnchantError> {
        Err(errors::EnchantError::NotEnchantable)
    }
    fn add_divinity(&mut self) -> Result<u16, errors::EnchantError> {
        Err(errors::EnchantError::CannotHoldDivinity)
    }
    fn remove_divnity(&mut self) -> Result<u16, errors::EnchantError> {
        Err(errors::EnchantError::CannotHoldDivinity)
    }
}

#[cfg(test)]
mod test {
    use super::{Gear, GearType};
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
    fn increase_enchant() {
        let mut gear = test_gear();
        let enchant = gear.enchant;
        // Increase Enchantment by 1
        gear.increase_enchant(1);
        // Enchantment should be 2
        assert_eq!(gear.enchant, enchant + 1);
        // Increase Enchantment to Max + 2
        gear.increase_enchant(255);
        // Enchantment Should be Max (and not error)
        assert_eq!(gear.enchant, 255);
    }

    #[test]
    fn decrease_enchant() {
        let mut gear = test_gear();
        assert_eq!(gear.enchant, 1);
        gear.decrease_enchant(1);
        assert_eq!(gear.enchant, 0);
        gear.decrease_enchant(255);
        // Make sure it wont Go past 0 / Error
        assert_eq!(gear.enchant, 0);
    }

    #[test]
    fn multipliyer() {
        let mut gear = test_gear();
        gear.enchant = 0;
        // Test gear.multipliyer()
        gear.enchant = 1;
        // Test gear.multipliyer()
        gear.is_boss = true;
        // Test gear.multipliyer()

        gear.enchant = 10;
        gear.accuracy = 100;
        gear.health = 100;
        gear.attack = 100;
        gear.defense = 100;
        gear.evasion = 100;
        gear.is_boss = false;

        // Test new gear.accuracy()
        // Test new gear.health()
        // Test new gear.attack()
        // Test new gear.defense()
        // Test new gear.evasion()

        gear.is_boss = true;
        // Test new gear.evasion()
        panic!("Gear Multiplyer Values arent ready!")
    }

}
