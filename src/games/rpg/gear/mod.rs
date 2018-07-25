use super::errors;
mod gear_items;
pub mod items;
pub use self::gear_items::GearItems;
pub use self::items::{Gear, GEAR_ITEMS, ID as GearID};
use failure::Error;
use serde_json;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Ord, PartialOrd, Copy)]
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

impl ::std::str::FromStr for GearType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use self::GearType::*;
        let string = s.to_uppercase();
        Ok(match string.as_str() {
            "weapon" => Weapon,
            "ArmorHead" => ArmorHead,
            "ArmorChest" => ArmorChest,
            "ArmorFeet" => ArmorFeet,
            "ArmorHands" => ArmorHands,
            "AccessoriesWrist" => AccessoriesWrist,
            "AccessoriesGloves" => AccessoriesGloves,
            "AccessoriesNeck" => AccessoriesNeck,
            _ => return Err(errors::GearTypeParseError::InvalidType { type_name: string }.into()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct GearInfoStore {
    id: GearID,
    pub enchant: u8,
    pub divinity: u16,
}

impl GearInfoStore {
    pub fn id(self) -> GearID {
        self.id
    }
    pub fn new(id: GearID, enchant: u8, divinity: u16) -> GearInfoStore {
        GearInfoStore {
            id,
            enchant,
            divinity,
        }
    }
}

impl ::std::fmt::Display for GearInfoStore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ::std::str::FromStr for GearInfoStore {
    type Err = errors::GearParseError;
    fn from_str(s: &str) -> Result<GearInfoStore, errors::GearParseError> {
        Ok(serde_json::from_str(s)?)
    }
}
