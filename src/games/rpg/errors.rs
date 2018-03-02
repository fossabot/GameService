use std::char::ParseCharError;
use std::error::Error as StdError;
use std::fmt;
#[cfg(feature = "auto_save")]
use diesel::result::Error as DieselResultError;
#[cfg(feature = "auto_save")]
use r2d2::Error as R2d2Error;
use std::convert::From;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum GearTypeParseError {
    InvalidType,
    CharParse(ParseCharError),
}

impl fmt::Display for GearTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for GearTypeParseError {
    fn description(&self) -> &str {
        use self::GearTypeParseError::*;
        match *self {
            InvalidType => "Not a valid Gear Type",
            CharParse(ref err) => err.description(),
        }
    }
}
#[derive(Debug)]
pub enum GearParseError {
    JsonError(SerdeJsonError),
}

impl From<SerdeJsonError> for GearParseError {
    fn from(error: SerdeJsonError) -> GearParseError {
        GearParseError::JsonError(error)
    }
}

impl fmt::Display for GearParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for GearParseError {
    fn description(&self) -> &str {
        use self::GearParseError::*;
        match *self {
            JsonError(ref err) => err.description(),
        }
    }
}

#[derive(Debug)]
pub enum PlayerError {
    #[cfg(feature = "auto_save")]
    DieselResult(DieselResultError),
    #[cfg(feature = "auto_save")]
    R2d2(R2d2Error),
    GearError(GearParseError),
}
impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for PlayerError {
    fn description(&self) -> &str {
        use self::PlayerError::*;
        match *self {
            #[cfg(feature = "auto_save")]
            R2d2(ref err) => err.description(),
            #[cfg(feature = "auto_save")]
            DieselResult(ref err) => err.description(),
            GearError(ref err) => err.description(),
        }
    }
}
#[cfg(feature = "auto_save")]
impl From<DieselResultError> for PlayerError {
    fn from(error: DieselResultError) -> PlayerError {
        PlayerError::DieselResult(error)
    }
}
#[cfg(feature = "auto_save")]
impl From<R2d2Error> for PlayerError {
    fn from(error: R2d2Error) -> PlayerError {
        PlayerError::R2d2(error)
    }
}
impl From<GearParseError> for PlayerError {
    fn from(error: GearParseError) -> PlayerError {
        PlayerError::GearError(error)
    }
}

#[derive(Debug)]
pub enum EnchantError {
    MaxEnchant,
    NotEnchantable,
    UnCursable,
    MaxCurses,
    NotHighEnoughEnchantment(u16),
}
impl fmt::Display for EnchantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for EnchantError {
    fn description(&self) -> &str {
        use self::EnchantError::*;
        match *self {
            MaxEnchant => "Item has reached maximum enchant.",
            NotEnchantable => "Item cannot be enchanted.",
            UnCursable => "Item cannot be cursed.",
            MaxCurses => "Item has reached its max curses.",
            NotHighEnoughEnchantment(_) => {
                "Item does not have enough enchantments to sacrifice for this action."
            }
        }
    }
}
#[derive(Debug)]
pub enum ShopError {
    NotEnoughFunds(u64),
    EnchantmentError(EnchantError),
}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for ShopError {
    fn description(&self) -> &str {
        use self::ShopError::*;
        match *self {
            NotEnoughFunds(_) => "You cannot afford this item",
            EnchantmentError(ref err) => err.description(),
        }
    }
}

impl From<EnchantError> for ShopError {
    fn from(err: EnchantError) -> ShopError {
        ShopError::EnchantmentError(err)
    }
}
