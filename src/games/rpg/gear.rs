use std::str::FromStr;
use std::u8;
use serde_json;
use std::fmt;
use super::errors::{GearParseError, GearTypeParseError};
#[derive(Debug, Deserialize, Serialize, Clone)]
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
    None,
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
            "None" => None,
            _ => return Err(GearTypeParseError::InvalidType),
        })
    }
}

/// Gear
/// Curesed items will have some negative stats
#[derive(Deserialize, Serialize, Clone)]
pub struct Gear {
    pub gear_type: GearType,
    /// Name of gear
    pub name: String,
    pub enchant: u8,
    pub _is_boss: bool,
    pub _accuracy: i64,
    pub _evasion: i64,
    pub _health: i64,
    pub _attack: i64,
    pub _defense: i64,
}

impl Gear {
    fn multipliyer(&self) -> f64 {
        if self._is_boss {
            f64::from(self.enchant) / 50f64
        } else {
            f64::from(self.enchant) / 100f64
        }
    }
    pub fn enchant_lvl(&self) -> u8 {
        self.enchant
    }
    pub fn increase_enchant(&mut self, amount: u8) {
        if self.enchant != u8::MAX {
            self.enchant = self.enchant.checked_add(amount).unwrap_or(u8::MAX);
        }
    }
    pub fn decrease_enchant(&mut self, amount: u8) {
        self.enchant = self.enchant.checked_sub(amount).unwrap_or(u8::MIN)
    }
    pub fn accuracy(&self) -> i64 {
        (self._accuracy as f64 * self.multipliyer()) as i64
    }
    pub fn evasion(&self) -> i64 {
        (self._evasion as f64 * self.multipliyer()) as i64
    }
    pub fn health(&self) -> i64 {
        (self._health as f64 * self.multipliyer()) as i64
    }
    pub fn attack(&self) -> i64 {
        (self._attack as f64 * self.multipliyer()) as i64
    }
    pub fn defense(&self) -> i64 {
        (self._defense as f64 * self.multipliyer()) as i64
    }
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
