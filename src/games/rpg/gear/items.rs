use super::{errors, GearInfoStore, GearItems, GearType};
use std::convert::Into;
use std::sync::RwLock;

pub type ID = u8;

const DIVINITY_BOOST: u16 = 1_000;

#[derive(Deserialize, Debug, Clone)]
pub struct LegalGear {
    name: String,
    gear_type: GearType,
    description: String,
    accuracy: i32,
    evasion: i32,
    health: i32,
    attack: i32,
    armor: i32,
    max_enchant: Option<u8>,
    gain_per_enchant: Option<f32>,
    max_divinity: Option<u16>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Gear {
    id: ID,
    name: String,
    gear_type: GearType,
    description: String,
    accuracy: i32,
    evasion: i32,
    health: i32,
    attack: i32,
    armor: i32,
    max_enchant: Option<u8>,
    gain_per_enchant: Option<f32>,
    max_divinity: Option<u16>,
    _enchantment_level: u8,
    _divinity: u16,
}

impl Gear {
    pub fn max_enchant(self) -> Result<u8, errors::EnchantError> {
        if let Some(max) = self.max_enchant {
            Ok(max)
        } else {
            Err(errors::EnchantError::MaxEnchant)
        }
    }
    pub fn enchant_lvl(&self) -> Result<u8, errors::EnchantError> {
        if let Some(max) = self.max_enchant {
            if max == 0 {
                Err(errors::EnchantError::NotEnchantable)
            } else {
                Ok(self._enchantment_level)
            }
        } else {
            Err(errors::EnchantError::NotEnchantable)
        }
    }

    pub fn increase_enchant(&mut self, amount: u8) -> Result<u8, errors::EnchantError> {
        let lvl = self.enchant_lvl()?;
        let max = self.max_enchant.unwrap();
        if lvl == max {
            Err(errors::EnchantError::MaxEnchant)
        } else if lvl + amount > max {
            self._enchantment_level = max;
            Ok(self._enchantment_level)
        } else {
            self._enchantment_level += amount;
            Ok(self._enchantment_level)
        }
    }

    pub fn decrease_enchant(&mut self, amount: u8) -> Result<u8, errors::EnchantError> {
        self.enchant_lvl()?;
        self._enchantment_level -= amount;
        Ok(self._enchantment_level)
    }

    pub fn divinity(&self) -> Result<u16, errors::EnchantError> {
        if self.enchant_lvl().is_ok() {
            if let Some(max) = self.max_divinity {
                if max == 0 {
                    Err(errors::EnchantError::CannotHoldDivinity)
                } else {
                    Ok(self._divinity)
                }
            } else {
                Err(errors::EnchantError::CannotHoldDivinity)
            }
        } else {
            Err(errors::EnchantError::CannotHoldDivinity)
        }
    }

    pub fn add_divinity(&mut self, amount: u16) -> Result<u16, errors::EnchantError> {
        let divinity = self.divinity()?;
        let max = self.max_divinity.unwrap();
        if divinity == max {
            Err(errors::EnchantError::MaxDivinity)
        } else if divinity + amount > max {
            self._divinity = max;
            Ok(self._divinity)
        } else {
            self._divinity += amount;
            Ok(self._divinity)
        }
    }

    pub fn remove_divnity(&mut self, amount: u16) -> Result<u16, errors::EnchantError> {
        self.divinity()?;
        self._divinity -= amount;
        Ok(self._divinity)
    }
    pub fn name(self) -> String {
        self.name
    }
    pub fn desc(self) -> String {
        self.description
    }
    pub fn pretty_text(&self) -> String {
        let mut base = format!(
            "Name: {}\nType: {:?}\nAttack: {}\nAccuracy: {}\nArmor: {}\nEvasion: {}\nHealth: {}\n",
            self.name,
            self.gear_type,
            pretty_num!(self.attack),
            pretty_num!(self.accuracy),
            pretty_num!(self.armor),
            pretty_num!(self.evasion),
            pretty_num!(self.health)
        );
        if let Ok(enchant) = self.enchant_lvl() {
            base.push_str(&format!(
                "\nEnchantment: {}/{}",
                pretty_num!(enchant),
                pretty_num!(self.max_enchant.unwrap_or_default())
            ));
        }
        if let Ok(divinity) = self.divinity() {
            base.push_str(&format!(
                "\nDivinity: {}/{}",
                pretty_num!(divinity),
                pretty_num!(self.max_divinity.unwrap_or_default())
            ))
        }
        base.push_str(&format!("\nDescription:\n{}", self.description));
        base
    }

    fn multiplyer(&self) -> f32 {
        self.gain_per_enchant.unwrap_or_default() * f32::from(self._enchantment_level)
            + f32::from(self._divinity) / f32::from(DIVINITY_BOOST)
    }
    pub fn accuracy(&self) -> i32 {
        self.accuracy + (self.accuracy as f32 * self.multiplyer()).floor() as i32
    }
    pub fn evasion(&self) -> i32 {
        self.evasion + (self.evasion as f32 * self.multiplyer()).floor() as i32
    }
    pub fn health(&self) -> i32 {
        self.health + (self.health as f32 * self.multiplyer()).floor() as i32
    }
    pub fn attack(&self) -> i32 {
        self.health + (self.health as f32 * self.multiplyer()).floor() as i32
    }
    pub fn armor(&self) -> i32 {
        self.armor + (self.armor as f32 * self.multiplyer()).floor() as i32
    }
    pub fn id(self) -> ID {
        self.id
    }
}

// TODO: Write a congiguration method for this
// TODO: Make it constant
lazy_static! {
    pub static ref GEAR_ITEMS: RwLock<GearItems> = RwLock::new(GearItems::new());
}

impl Into<Result<Gear, errors::GearParseError>> for GearInfoStore {
    fn into(self) -> Result<Gear, errors::GearParseError> {
        let v = match GEAR_ITEMS.write() {
            Ok(mut g) => {
                if g.path().is_none() {
                    g.load_dir(::std::path::PathBuf::from("RPG_ITEMS/GEAR"));
                }
                Ok(g.get(&self.id)?.clone())
            }
            Err(_) => Err(errors::GearParseError::DoesNotExist { id: self.id }),
        }?;
        Ok(Gear {
            id: self.id,
            name: v.name.clone(),
            gear_type: v.gear_type,
            description: v.description.clone(),
            accuracy: v.accuracy,
            evasion: v.evasion,
            health: v.health,
            attack: v.attack,
            armor: v.armor,
            max_enchant: v.max_enchant,
            gain_per_enchant: v.gain_per_enchant,
            max_divinity: v.max_divinity,
            _enchantment_level: self.enchant,
            _divinity: self.divinity,
        })
    }
}

impl Into<Result<LegalGear, errors::GearParseError>> for GearInfoStore {
    fn into(self) -> Result<LegalGear, errors::GearParseError> {
        match GEAR_ITEMS.write() {
            Ok(mut g) => {
                if g.path().is_none() {
                    g.load_dir(::std::path::PathBuf::from("RPG_ITEMS/GEAR"));
                }
                Ok(g.get(&self.id)?.clone())
            }
            Err(_) => Err(errors::GearParseError::DoesNotExist { id: self.id }),
        }
    }
}

impl Into<GearInfoStore> for Gear {
    fn into(self) -> GearInfoStore {
        GearInfoStore {
            id: self.id,
            enchant: self._enchantment_level,
            divinity: self._divinity,
        }
    }
}

impl ::std::cmp::PartialEq for Gear {
    fn eq(&self, rhs: &Gear) -> bool {
        self.id == rhs.clone().id()
    }
}

impl ::std::cmp::Eq for Gear {}

impl ::std::cmp::PartialOrd for Gear {
    fn partial_cmp(&self, rhs: &Gear) -> Option<::std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self.eq(&rhs) {
            Some(Ordering::Equal)
        } else {
            match self.gear_type.partial_cmp(&rhs.gear_type) {
                None => if self.id > rhs.clone().id() {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                },
                Some(order) => match order {
                    Ordering::Equal => if self.id > rhs.clone().id() {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    },
                    _ => Some(order),
                },
            }
        }
    }
}

impl ::std::cmp::Ord for Gear {
    fn cmp(&self, rhs: &Gear) -> ::std::cmp::Ordering {
        self.partial_cmp(&rhs).unwrap()
    }
}

impl GearInfoStore {
    fn get_gear(&self) -> Result<LegalGear, errors::GearParseError> {
        (*self).into()
    }
    pub fn max_enchant(self) -> Result<Result<u8, errors::EnchantError>, errors::GearParseError> {
        let g = self.get_gear()?;
        Ok(if let Some(max) = g.max_enchant {
            Ok(max)
        } else {
            Err(errors::EnchantError::NotEnchantable)
        })
    }
    pub fn enchant_lvl(&self) -> Result<Result<u8, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.max_enchant()? {
            Ok(max) => Ok(if self.enchant > max {
                max
            } else {
                self.enchant
            }),
            Err(e) => Err(e),
        })
    }
    pub fn increase_enchant(
        &mut self,
        amount: u8,
    ) -> Result<Result<u8, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.max_enchant()? {
            Ok(max) => if self.enchant >= max {
                Err(errors::EnchantError::MaxEnchant)
            } else {
                Ok({
                    let amount = self.enchant.checked_add(amount).unwrap_or(max);
                    self.enchant = if amount > max { max } else { amount };
                    self.enchant
                })
            },
            Err(e) => Err(e),
        })
    }
    pub fn decrease_enchant(
        &mut self,
        amount: u8,
    ) -> Result<Result<u8, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.enchant_lvl()? {
            Ok(_) => {
                self.enchant = self.enchant.checked_sub(amount).unwrap_or(0);
                Ok(self.enchant)
            }
            Err(e) => Err(e),
        })
    }
    pub fn max_divinity(
        &self,
    ) -> Result<Result<u16, errors::EnchantError>, errors::GearParseError> {
        let g = self.get_gear()?;
        Ok(match g.max_divinity {
            Some(max) => Ok(max),
            None => Err(errors::EnchantError::CannotHoldDivinity),
        })
    }
    pub fn divinity(&self) -> Result<Result<u16, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.max_divinity()? {
            Ok(max) => Ok(if self.divinity > max {
                max
            } else {
                self.divinity
            }),
            Err(e) => Err(e),
        })
    }
    pub fn add_divinity(
        &mut self,
        amount: u16,
    ) -> Result<Result<u16, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.max_divinity()? {
            Ok(max) => {
                if self.divinity >= max {
                    Err(errors::EnchantError::MaxEnchant)
                } else {
                    Ok({
                        let amount = self.divinity.checked_add(amount).unwrap_or(max);
                        self.divinity = if amount >= max { max } else { amount };
                        self.divinity
                    })
                }
            }
            Err(e) => Err(e),
        })
    }
    pub fn remove_divinity(
        &mut self,
        amount: u16,
    ) -> Result<Result<u16, errors::EnchantError>, errors::GearParseError> {
        Ok(match self.divinity()? {
            Ok(_) => {
                self.divinity = self.divinity.checked_sub(amount).unwrap_or(0);
                Ok(self.divinity)
            }
            Err(e) => Err(e),
        })
    }
    pub fn name(self) -> Result<String, errors::GearParseError> {
        Ok(self.get_gear()?.name)
    }
    pub fn description(self) -> Result<String, errors::GearParseError> {
        Ok(self.get_gear()?.description)
    }
}
#[cfg(test)]
mod test {
    use super::GEAR_ITEMS;
    #[test]
    fn gear_items() {
        GEAR_ITEMS.read().ok();
        if GEAR_ITEMS.read().unwrap().is_empty() {
            panic!("{:?}", GEAR_ITEMS.read().unwrap().path().unwrap().display());
        }
    }
}
