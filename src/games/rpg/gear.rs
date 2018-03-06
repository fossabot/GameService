use std::str::FromStr;
use std::u8;
use serde_json;
use std::fmt;
use super::errors::{GearParseError, GearTypeParseError};
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
        (self.accuracy as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `evasion`  by enchantment lvl and returns it
    pub fn evasion(&self) -> i64 {
        (self.evasion as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `health` by enchantment lvl and returns it
    pub fn health(&self) -> i64 {
        (self.health as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `attack` by enchantment lvl and returns it
    pub fn attack(&self) -> i64 {
        (self.attack as f64 * self.multipliyer()) as i64
    }
    /// Adjusts the `defense` by enchantment lvl and returns it
    pub fn defense(&self) -> i64 {
        (self.defense as f64 * self.multipliyer()) as i64
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
