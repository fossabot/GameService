use super::enchant::Enchanter;
use super::errors::{EnchantError, ShopError};
use super::Gear;
pub struct Shop;

impl Shop {
    /// Calculate the gear price based on enchantment lvl
    pub fn enchant_price(gear: &Gear) -> Result<u64, ShopError> {
        if gear.enchant == 255 {
            Err(EnchantError::MaxEnchant.into())
        } else {
            Ok((u64::from(gear.enchant) + 1).pow(4) * 100)
        }
    }
    const REROLL_PRICE: u64 = 2_531_250;
    const CURSE_PRICE: u64 = 5_248_800;

    /// Enchant Gear for a price
    pub fn enchant_gear(g: &mut Gear, funds: &mut u64) -> Result<bool, ShopError> {
        // Enchantment Lvl
        let level = g.enchant;
        // Price to perform enchant
        let price = Shop::enchant_price(g)?;
        if price > *funds {
            Err(ShopError::NotEnoughFunds(price))
        } else {
            *funds -= price;
            Enchanter::enchant_gear(g)?;
            // Checks if enchantment lvl is greater than current lvl
            Ok(g.enchant > level)
        }
    }

    /// Reroll a gear's lowest stat for a price
    pub fn reroll_gear(g: &mut Gear, funds: &mut u64) -> Result<(), ShopError> {
        if Shop::REROLL_PRICE > *funds {
            return Err(ShopError::NotEnoughFunds(Shop::REROLL_PRICE));
        }
        Enchanter::reroll_stats(g)?;
        *funds -= Shop::REROLL_PRICE;
        Ok(())
    }

    // Curses a random stat on the gear for a price
    pub fn curse_gear(g: &mut Gear, funds: &mut u64) -> Result<(), ShopError> {
        if Shop::CURSE_PRICE > *funds {
            return Err(ShopError::NotEnoughFunds(Shop::CURSE_PRICE));
        }
        Enchanter::curse_gear(g)?;
        *funds -= Shop::CURSE_PRICE;
        Ok(())
    }

    /// Returns the odds of success as a precentage
    pub fn get_enchant_odds(enchant_lvl: u8) -> Result<String, ShopError> {
        Ok(format!("{}%", Enchanter::odds(enchant_lvl)? * 100.0))
    }
}
