use super::enchant::Enchanter;
use super::errors;
use super::errors::{EnchantError, ShopError};
use super::Gear;
pub struct Shop;

use failure::Error;

impl Shop {
    /// Calculate the gear price based on enchantment lvl
    pub fn enchant_price(gear: &Gear) -> Result<u64, EnchantError> {
        let lvl = gear.enchant_lvl()?;
        if gear.clone().max_enchant()? == lvl {
            Err(EnchantError::MaxEnchant)
        } else {
            Ok((u64::from(lvl) + 1).pow(4) * 100)
        }
    }
    // const REROLL_PRICE: u64 = 2_531_250;
    // const CURSE_PRICE: u64 = 5_248_800;

    /// Enchant Gear for a price
    pub fn enchant_gear(g: &mut Gear, funds: &mut u64) -> Result<bool, Error> {
        // Enchantment Lvl
        if let Ok(level) = g.enchant_lvl() {
            let price = Shop::enchant_price(g)?;
            if price > *funds {
                Err(ShopError::NotEnoughFunds {
                    amount: price,
                    balance: *funds,
                }.into())
            } else {
                *funds -= price;
                Enchanter::enchant_gear(g)?;
                Ok(g.enchant_lvl().unwrap() > level)
            }
        } else {
            Err(errors::EnchantError::NotEnchantable.into())
        }
    }

    // Removed: Gear can no longer be re-rolled
    // /// Reroll a gear's lowest stat for a price
    // pub fn reroll_gear(g: &mut Gear, funds: &mut u64) -> Result<(), Error> {}

    // Removed: Gear can no longer be cursed
    // /// Curses a random stat on the gear for a price
    // pub fn curse_gear(g: &mut Gear, funds: &mut u64) -> Result<(), ShopError> {}

    /// Returns the odds of success as a precentage
    pub fn get_enchant_odds(enchant_lvl: u8) -> Result<String, Error> {
        Ok(format!("{}%", Enchanter::odds(enchant_lvl)? * 100.0))
    }
}
