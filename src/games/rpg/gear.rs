use super::errors::{GearParseError, GearTypeParseError};
use serde_json;
use std::fmt;
use std::str::FromStr;
use std::u8;
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum GearType {
    Weapon,
    ArmorHead,
    ArmorChest,
    ArmorFeet,
    ArmorHands,
    AccessoriesWrist,
    AccessoriesHead,
    AccessoriesGloves,
    AccessoriesNeck,
}

impl FromStr for GearType {
    type Err = GearTypeParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use self::GearType::*;
        let string = s.to_uppercase();
        Ok(match string.as_str() {
            "weapon" => Weapon,
            "armor_head" => ArmorHead,
            "armor_chest" => ArmorChest,
            "armor_feet" => ArmorFeet,
            "armor_hands" => ArmorHands,
            "accessories_wrist" => AccessoriesWrist,
            "accessories_gloves" => AccessoriesGloves,
            "accessories_neck" => AccessoriesNeck,
            _ => return Err(GearTypeParseError::InvalidType),
        })
    }
}

/// Gear
/// Curesed items will have some negative stats
#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gear {
    pub gear_type: GearType,
    /// Name of gear
    pub name: String,
    pub enchant: u8,
    pub is_boss: bool,
    pub accuracy: i64,
    pub evasion: i64,
    pub health: i64,
    pub attack: i64,
    pub defense: i64,
}

impl Gear {
    fn multipliyer(&self) -> f64 {
        if self.is_boss {
            f64::from(self.enchant) / 50f64
        } else {
            f64::from(self.enchant) / 100f64
        }
    }

    /// Increase to enchantment (Called by `Enchanter`)
    pub fn increase_enchant(&mut self, amount: u8) {
        if self.enchant != u8::MAX {
            self.enchant = self.enchant.checked_add(amount).unwrap_or(u8::MAX);
        }
    }
    /// Decrease enchantment (Called by `Enchanter`)
    pub fn decrease_enchant(&mut self, amount: u8) {
        self.enchant = self.enchant.checked_sub(amount).unwrap_or(u8::MIN)
    }
    /// Adjusts the `accuracy` stat by enchantment lvl and returns it
    pub fn accuracy(&self) -> i64 {
        self.accuracy + (self.accuracy as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `evasion`  by enchantment lvl and returns it
    pub fn evasion(&self) -> i64 {
        self.evasion + (self.evasion as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `health` by enchantment lvl and returns it
    pub fn health(&self) -> i64 {
        self.health + (self.health as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `attack` by enchantment lvl and returns it
    pub fn attack(&self) -> i64 {
        self.attack + (self.attack as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `defense` by enchantment lvl and returns it
    pub fn defense(&self) -> i64 {
        self.defense + (self.defense as f64 * self.multipliyer()) as i64
    }
    /// The name of the gear
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Gear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_value(self.clone())
            .unwrap_or_default()
            .to_string();
        f.write_str(&json)
    }
}

impl FromStr for Gear {
    type Err = GearParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(serde_json::from_str(s)?)
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
