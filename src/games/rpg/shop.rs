use super::enchant::Enchanter;
use super::errors::{EnchantError, ShopError};
use super::Gear;
pub struct Shop;

impl Shop {
    pub fn enchant_price(gear: &Gear) -> Result<u64, ShopError> {
        if gear.enchant == 255 {
            Err(EnchantError::MaxEnchant.into())
        } else {
            Ok((gear.enchant as u64 + 1).pow(4) * 100)
        }
    }
    const REROLL_PRICE: u64 = 2_531_250;
    const CURSE_PRISE: u64 = 5_248_800;

    pub fn enchant_gear(g: &mut Gear, funds: u64) -> Result<(Gear, u64, bool), ShopError> {
        let level = g.enchant;
        let price = Shop::enchant_price(g)?;
        if price > funds {
            Err(ShopError::NotEnoughFunds(price))
        } else {
            let gear = Enchanter::enchant_gear(g)?;
            let success = gear.enchant == level;
            Ok((gear, funds - price, success))
        }
    }
    pub fn reroll_gear(g: &mut Gear, funds: u64) -> Result<(Gear, u64), ShopError> {
        if Shop::REROLL_PRICE > funds {
            return Err(ShopError::NotEnoughFunds(Shop::REROLL_PRICE));
        }
        let gear = Enchanter::reroll_stats(g)?;
        Ok((gear, funds-Shop::REROLL_PRICE))
    }
    pub fn curse_gear(g: &mut Gear, funds: u64) -> Result<(Gear, u64), ShopError> {
        if Shop::CURSE_PRISE > funds {
            return Err(ShopError::NotEnoughFunds(Shop::CURSE_PRISE));
        }
        let gear = Enchanter::curse_gear(g)?;
        Ok((gear, funds - Shop::CURSE_PRISE))
    }
    pub fn get_enchant_odds(enchant_lvl: u8) -> Result<String, ShopError> {

        Ok(format!("{}%", Enchanter::odds(enchant_lvl)? * 100.0))
    }
}
