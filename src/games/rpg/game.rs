use super::errors::GameError;
use super::{Dungeon, Gear, Player, Shop};
#[cfg(feature = "auto_save")]
use ConnectionPool;

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
    ) -> Result<Self, GameError> {
        Ok(Self {
            player: Player::get(player_id, conn_pool)?,
            funds,
            log: vec![],
        })
    }

    /// Create a new Game session
    #[cfg(not(feature = "auto_save"))]
    pub fn new(player: Player, funds: u64) -> Result<Self, GameError> {
        Ok(Self {
            player,
            funds,
            log: vec![],
        })
    }

    /// de-increment the target floor to the closest devisable of 5 and run the next 5 floors
    pub fn run_floors(&mut self, target_floor: u64, auto_buy: bool) {
        let results = Dungeon::challange(target_floor, self.player.clone(), self.funds, auto_buy);
        self.funds = results.0;
        self.log.push(results.1);
        self.player = results.2;
    }

    /// Show the gear of the player
    pub fn show_gear(&self) -> Vec<Gear> {
        let mut gear = self.player.gear.clone();
        gear.sort();
        gear
    }

    /// Enchant the specified gear of the player (use `show_gear` to get the gear in order)
    pub fn enchant_gear(&mut self, gear: usize) -> String {
        self.player.gear.sort();
        let mut gear = self.player.gear.remove(gear);
        match Shop::enchant_gear(&mut gear, &mut self.funds) {
            Ok(success) => {
                self.player.gear.push(gear.clone());
                self.player.gear.sort();
                let msg = if success {
                    format!("Successfully enchanted {}!", gear)
                } else {
                    format!("Failed to enchant {}!", gear)
                };
                self.log.push(msg.clone());
                msg
            }
            Err(why) => {
                let msg = format!("Enchantment Error: {}", why);
                // warn!("{}", msg);
                self.log.push(msg.clone());
                msg
            }
        }
    }

    /// Returns the % chance of enchant success as a string
    pub fn enchant_odds(&self, gear: usize) -> Result<String, GameError> {
        let enchant = self.player.gear[gear].enchant;
        Ok(Shop::get_enchant_odds(enchant)?)
    }

    /// Curses gear, returns result as a string
    pub fn curse_gear(&mut self, gear: usize) -> String {
        self.player.gear.sort();
        let mut g = self.player.gear.remove(gear);
        let msg = match Shop::curse_gear(&mut g, &mut self.funds) {
            Ok(_) => {
                self.player.gear.push(g.clone());
                format!("Successfully cursed {}!", g)
            }
            Err(why) => {
                let msg = format!("Failed to curse {}, {}", g, why);
                // warn!("{}", why);
                msg
            }
        };
        self.log.push(msg.clone());
        msg
    }

    /// Reroll gear stats
    pub fn reroll_gear(&mut self, gear: usize) -> String {
        let mut g = self.player.gear.remove(gear);
        let msg = match Shop::reroll_gear(&mut g, &mut self.funds) {
            Ok(_) => {
                self.player.gear.push(g.clone());
                format!("Successfully re-rolled {}", g)
            }
            Err(why) => {
                let msg = format!("Failed to re-roll, {}", why);
                // warn!("{}", msg);
                msg
            }
        };
        self.log.push(msg.clone());
        msg
    }
}

/// Auto Save
#[cfg(feature = "auto_save")]
impl Drop for Game {
    fn drop(&mut self) {
        self.player.save().expect("Failed to save");
    }
}
