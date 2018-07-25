#[macro_export]
macro_rules! unenchantable_display {
    ($name:ident) => {
        impl PrettyFMT for $name {
            fn pretty_format(&self) -> String {
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
    }
}
#[macro_export]
macro_rules! enchantable_display {
    ($name:ident) => {
        impl PrettyFMT for $name {
            fn pretty_textify(&self) -> String {
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
    }
}

#[macro_export]
macro_rules! gear_attr {
    (Attack => $atk:expr) => {
        const ATTACK: i64 = $atk;
    };
    (Armor => $armor:expr) => {
        const ARMOR: i64 = $armor;
    };
    (Accuracy => $acc:expr) => {
        const ACCURACY: i64 = $acc;
    };
    (Evasion => $evas:expr) => {
        const EVASION: i64 = $evas;
    };
    (Health => $hp:expr) => {
        const HEALTH: i64 = $hp;
    };
    (Description => $desc:expr) => {
        const DESCRIPTION: &'static str = $desc;
    };
    (MaxEnchant => $max:expr) => {
        const MAX_ENCHANT: u8 = $max;
    };
    (GainPerEnchant => $gain:expr) => {
        const GAIN_PER_ENCHANT: f32 = $gain;
    };
    (GearType => $gear_type:expr) => {
        const GEAR_TYPE: GearType = $gear_type;
    };
    (Name => $name:expr) => {
        const NAME: &'static str = $name;
    };
}

#[macro_export]
macro_rules! gear {
    ($($name:expr => $(key:expr => $value:expr),+);*) => {
        $(
            #[derive(Debug, Deserialize, Seralize, Clone)]
            pub struct $name {
                _enchantment_level: 0
                _divinity: 0
            }
            impl Gear for $name {
                $(
                    gear_attr!($key => $value)
                )*
                fn enchant_lvl(&self) -> u8 {
                    self._enchant_lvl
                }
                fn divinity(&self) -> u16 {
                    self._divinity
                }
                fn increase_enchant(&mut self) -> Result<u8, EnchantError> {
                    if Self::MAX_ENCHANT == 0 {
                        Err(EnchantError::NotEnchantable)
                    }
                    else if Self::MAX_ENCHANT == self._enchant_lvl {
                        Err(EnchantError::MaxEnchant)
                    } else {
                        self._enchant_lvl += 1;
                        Ok(self._enchant_lvl)
                    }
                }
                fn decrease_enchant(&mut self) -> u8 {
                    if self._enchant_lvl > 0 {
                        self._enchant_lvl -= 1;
                    }
                    self._enchant_lvl
                }
                fn add_divinity(&mut self) -> Result<u16, EnchantError> {
                    if (Self::MAX_ENCHANT == 0 ) || (Self::MAX_DIVINITY == 0) {
                        Err(EnchantError::CannotHoldDivinity)
                    } else {
                        self._divinity +=1;
                        Ok(self._divinity)
                    }
                }
                fn remove_divinity(&mut self) -> u16 {
                    if self._divinity != 0 {
                        self._divinity -= 1;
                    }
                    self._divinity
                }
            }
        )*
    }
}
