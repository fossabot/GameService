use super::errors;
use super::gear::GearID;
use super::{Dungeon, Gear, Player, Shop};
#[cfg(feature = "auto_save")]
use ConnectionPool;

use failure::Error;

pub struct Game {
    player: Player,
    funds: u64,
    log: Vec<String>,
}

impl Game {
    /// Export the game info (Player, funds, log)
    #[cfg(not(feature = "auto_save"))]
    pub fn export(self) -> (Player, u64, Vec<String>) {
        (self.player, self.funds, self.log)
    }

    /// Fetch the player from the database and create a new game session
    #[cfg(feature = "auto_save")]
    pub fn new(
        player_id: Option<u64>,
        conn_pool: Option<&ConnectionPool>,
        funds: u64,
    ) -> Result<Self, Error> {
        Ok(Self {
            player: Player::get(player_id, conn_pool)?,
            funds,
            log: vec![],
        })
    }

    /// Create a new Game session
    #[cfg(not(feature = "auto_save"))]
    pub fn new(player: Player, funds: u64) -> Result<Self, Error> {
        Ok(Self {
            player,
            funds,
            log: vec![],
        })
    }

    /// de-increment the target floor to the closest devisable of 5 and run the next 5 floors
    pub fn run_floors(&mut self, target_floor: u32, auto_buy: bool) {
        let results = Dungeon::challange(target_floor, self.player.clone(), self.funds, auto_buy);
        self.funds = results.0;
        self.log.push(results.1);
        self.player = results.2;
    }

    /// Show the gear of the player
    pub fn show_gear(&self) -> Vec<Gear> {
        let mut gear: Vec<Gear> = self
            .player
            .gear
            .iter()
            .filter_map(|g| {
                let g: Result<Gear, errors::GearParseError> = (*g).into();
                g.ok()
            })
            .collect();
        gear.sort();
        gear
    }

    pub fn get_gear(&self, pos: usize) -> Result<Gear, Error> {
        match self.show_gear().get(pos) {
            Some(g) => Ok(g.clone()),
            None => Err(errors::GearParseError::DoesNotExist { id: 0 }.into()),
        }
    }

    /// Enchant the specified gear of the player (use `show_gear` to get the gear in order)
    pub fn enchant_gear(&mut self, gear_id: GearID) -> String {
        self.player
            .gear
            .iter()
            .position(|ref g| g.id() == gear_id)
            .and_then(|pos| {
                if let Ok(mut gear) = self.player.gear.remove(pos).into() {
                    match Shop::enchant_gear(&mut gear, &mut self.funds) {
                        Ok(success) => {
                            self.player.gear.push(gear.clone().into());
                            let msg = if success {
                                format!("Successfully enchanted {}!", gear.name())
                            } else {
                                format!("Failed to enchant {}!", gear.name())
                            };
                            self.log.push(msg.clone());
                            Some(msg)
                        }
                        Err(why) => {
                            let msg = format!("Enchantment Error: {}", why);
                            // warn!("{}", msg);
                            self.log.push(msg.clone());
                            Some(msg)
                        }
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| format!("GearID Does not exist: {}", gear_id))
    }

    /// Returns the % chance of enchant success as a string
    pub fn enchant_odds(&self, gear: usize) -> Result<String, Error> {
        let gear = self.get_gear(gear)?;
        Shop::get_enchant_odds(gear.enchant_lvl()?)
    }

    // Removed: Curses removed for balancing
    // /// Curses gear, returns result as a string
    // pub fn curse_gear(&mut self, gear: usize) -> String {}

    // Removed: Gear re-rolls was removed (Balance is nice)
    // /// Reroll gear stats
    // pub fn reroll_gear(&mut self, gear: usize) -> String {}
}

/// Auto Save
#[cfg(feature = "auto_save")]
impl Drop for Game {
    fn drop(&mut self) {
        self.player.save().expect("Failed to save");
    }
}
