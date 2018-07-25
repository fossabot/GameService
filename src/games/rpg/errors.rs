use super::gear::GearID;
#[cfg(feature = "auto_save")]
use diesel::result::Error as DieselResultError;
#[cfg(feature = "auto_save")]
use r2d2::Error as R2d2Error;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Fail)]
pub enum GearTypeParseError {
    #[fail(display = "Invalid type: {}", type_name)]
    InvalidType { type_name: String },
    #[fail(display = "Failed to parse json: {}", err)]
    JsonError { err: Box<SerdeJsonError> },
}

#[derive(Debug, Fail)]
pub enum GearParseError {
    #[fail(display = "Failed to read Json")]
    JsonError { err: Box<SerdeJsonError> },
    #[fail(display = "Invalid ID: {}", id)]
    DoesNotExist { id: GearID },
}

impl From<SerdeJsonError> for GearParseError {
    fn from(error: SerdeJsonError) -> GearParseError {
        GearParseError::JsonError {
            err: Box::new(error),
        }
    }
}

// impl fmt::Display for GearParseError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.write_str(self.description())
//     }
// }

// impl StdError for GearParseError {
//     fn description(&self) -> &str {
//         use self::GearParseError::*;
//         match *self {
//             JsonError(ref err) => err.description(),
//         }
//     }
// }

#[derive(Debug, Fail)]
pub enum PlayerError {
    #[cfg(feature = "auto_save")]
    #[fail(display = "{}", err)]
    DieselResult { err: Box<DieselResultError> },
    #[cfg(feature = "auto_save")]
    #[fail(display = "{}", err)]
    R2d2 { err: Box<R2d2Error> },
    #[fail(display = "{}", err)]
    GearError { err: GearParseError },
    #[cfg(feature = "auto_save")]
    #[fail(display = "No Connection Pool")]
    NoConnectionPool,
    #[cfg(feature = "auto_save")]
    #[fail(display = "No ID")]
    NoID,
    #[cfg(feature = "auto_save")]
    #[fail(display = "Attempted to save when configured not to")]
    DoNotSaveConfig,
}

#[cfg(feature = "auto_save")]
impl From<DieselResultError> for PlayerError {
    fn from(error: DieselResultError) -> PlayerError {
        PlayerError::DieselResult {
            err: Box::new(error),
        }
    }
}

#[cfg(feature = "auto_save")]
impl From<R2d2Error> for PlayerError {
    fn from(error: R2d2Error) -> PlayerError {
        PlayerError::R2d2 {
            err: Box::new(error),
        }
    }
}

impl From<GearParseError> for PlayerError {
    fn from(error: GearParseError) -> PlayerError {
        PlayerError::GearError { err: error }
    }
}

#[derive(Debug, Fail)]
pub enum EnchantError {
    #[fail(display = "Item has reached its max enchantment level")]
    MaxEnchant,
    #[fail(display = "Item is not enchantable")]
    NotEnchantable,
    #[fail(
        display = "Not High enough enchantment level for this action, required level {}.",
        req
    )]
    NotHighEnoughEnchantment { req: u16 },
    #[fail(display = "Not Enough Divinity")]
    NotEnoughDivinity,
    #[fail(display = "Cannot hold Divinity")]
    CannotHoldDivinity,
    #[fail(display = "Max Divinity")]
    MaxDivinity,
}

#[derive(Debug, Fail)]
pub enum ShopError {
    #[fail(
        display = "Not enough funds for this action. Requires {} but your balance is {}.",
        amount,
        balance
    )]
    NotEnoughFunds { balance: u64, amount: u64 },
    #[fail(display = "{}", err)]
    EnchantError { err: EnchantError },
}

impl From<EnchantError> for ShopError {
    fn from(err: EnchantError) -> ShopError {
        ShopError::EnchantError { err }
    }
}
