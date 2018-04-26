#[cfg(feature = "auto_save")]
use super::errors::PlayerError;
use super::Gear;
#[cfg(feature = "auto_save")]
use diesel;
#[cfg(feature = "auto_save")]
use diesel::prelude::*;
#[cfg(feature = "auto_save")]
use ConnectionPool;

use std::i64;

#[derive(Clone)]
pub struct Player {
    #[cfg(feature = "auto_save")]
    id: Option<u64>,
    pub exp: u64,
    pub damage_recieved: i64,
    pub gear: Vec<Gear>,
    #[cfg(feature = "auto_save")]
    conn_pool: Option<ConnectionPool>,
    #[cfg(feature = "auto_save")]
    save: bool,
}

impl Player {
    /// Health, this value is reduced when `DEFENSE` is 0
    const HEALTH: u64 = 100;
    /// Attack Stat, This stat effects attack damage
    const ATTACK: i64 = 10;
    /// Defense stat, 1:1 reduction of damage recieved
    const DEFENSE: i64 = 100;
    /// Evasion stat, this stat effections your chance of dodging an attack (Not Implemented)
    const EVASION: i64 = 1;
    /// Accuracy stat, this stat effects your chance of hitting a target countering evasion `ACCURACY`:`EVASION` 3:1 (Not Implmented)
    const ACCURACY: i64 = 5;
    #[cfg(feature = "auto_save")]
    pub fn get(player_id: Option<u64>, pool: Option<&ConnectionPool>) -> Result<Self, PlayerError> {
        if player_id.is_none() || pool.is_none() {
            return Ok(Self {
                id: None,
                exp: 0,
                damage_recieved: 0,
                gear: vec![],
                conn_pool: None,
                save: false,
            });
        };
        // None should be unreachable
        let player_id = player_id.unwrap();
        let pool = pool.unwrap();

        use models::RPGSession;
        use schema::rpgplayer as rpgplayer_schema;
        use schema::rpgplayer::dsl::*;
        let conn = pool.get()?;
        let mut results = rpgplayer
            .filter(id.eq(player_id as i64))
            .limit(1)
            .load::<RPGSession>(&*conn)?;
        let sess = if !results.is_empty() {
            results.remove(0)
        } else {
            let sess = RPGSession {
                id: player_id as i64,
                exp: 0,
                damage_recieved: 0,
                gear: vec![],
            };
            diesel::insert_into(rpgplayer_schema::table)
                .values(&sess)
                .execute(&*conn)?;
            sess
        };
        Ok(Self {
            id: Some(sess.id as u64),
            exp: sess.exp as u64,
            damage_recieved: sess.damage_recieved,
            gear: c![g.parse()?, for g in sess.gear],
            conn_pool: Some(pool.clone()),
            save: true,
        })
    }
    #[cfg(not(feature = "auto_save"))]
    pub fn get() -> Self {
        Self {
            exp: 0,
            damage_recieved: 0,
            gear: vec![],
        }
    }
    pub fn level(&self) -> u64 {
        self.exp / 10_000
    }
    fn multiplyer(&self) -> f64 {
        self.level() as f64 / 80f64
    }
    fn base_health(&self) -> u64 {
        Self::HEALTH + (Self::HEALTH as f64 * self.multiplyer()) as u64
    }
    pub fn current_health(&self) -> i64 {
        let health: i64 = self.gear
            .iter()
            .fold(self.base_health() as i64, |base, gear| {
                let health = gear.health();
                base.checked_add(gear.health())
                    .unwrap_or(if health > 0 { i64::MAX } else { 0 })
            });
        if health < 0 {
            0
        } else {
            let def = self.defense();
            let recieved_damge = if self.damage_recieved < def as i64 {
                0
            } else {
                self.damage_recieved
                    .checked_sub(self.defense() as i64)
                    .unwrap_or(0)
            };
            health.checked_sub(recieved_damge).unwrap_or(0)
        }
    }
    pub fn accuracy(&self) -> i64 {
        let base = Self::ACCURACY + (Self::ACCURACY as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| base + gear.accuracy())
    }
    pub fn evasaion(&self) -> i64 {
        let base = Self::EVASION + (Self::EVASION as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| base + gear.evasion())
    }
    pub fn attack(&self) -> i64 {
        let base = Self::ATTACK + (Self::ATTACK as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| gear.attack() + base)
    }
    pub fn defense(&self) -> u64 {
        let base = Self::DEFENSE + (Self::DEFENSE as f64 * self.multiplyer()) as i64;
        let def = self.gear
            .iter()
            .fold(base, |base, gear| base + gear.defense());
        if def < 0 {
            0
        } else {
            def as u64
        }
    }
    pub fn resurect(&mut self) {
        self.damage_recieved = 0;
    }
    pub fn heal(&mut self, amount: i64) {
        self.damage_recieved -= amount;
    }
    pub fn is_alive(&self) -> bool {
        self.current_health() >= 1
    }
    pub fn do_attack(&self) -> u64 {
        let dmg = self.attack();
        if dmg < 0 {
            0
        } else {
            dmg as u64
        }
    }
    /// Recieve Damage
    pub fn recieve_damage(&mut self, amount: u64) -> u64 {
        self.damage_recieved = self.damage_recieved
            .checked_add(amount as i64)
            .unwrap_or(i64::MAX);
        self.damage_recieved as u64
    }
    #[cfg(feature = "auto_save")]
    pub fn save(&self) -> Result<(), PlayerError> {
        use models::RPGSession;
        match self.conn_pool {
            Some(ref pool) => {
                let conn = pool.get()?;
                match self.id {
                    Some(id) => {
                        if !self.save {
                            return Err(PlayerError::DoNotSaveConfig);
                        }
                        let sess = RPGSession {
                            id: id as i64,
                            gear: c![g.to_string(), for g in &self.gear],
                            damage_recieved: self.damage_recieved,
                            exp: self.exp as i64,
                        };
                        sess.save_changes::<RPGSession>(&*conn)?;
                        Ok(())
                    }
                    None => Err(PlayerError::NoID),
                }
            }
            None => Err(PlayerError::NoConnectionPool),
        }
    }
    pub fn status(&self) -> String {
        let def = self.defense();
        let new_def = def.checked_sub(self.damage_recieved as u64).unwrap_or(0);
        let damage_recieved: u64 = (self.damage_recieved as u64).checked_sub(def).unwrap_or(0);
        let health = self.base_health();
        let new_health = health - damage_recieved;
        let level = self.level();
        let exp = self.exp - (level * 10_000);
        format!(
            "Armor: {}/{}\nHealth: {}/{}\nEXP: {}/10,000\nLevel: {}",
            new_def, def, new_health, health, exp, level
        )
    }
}

#[cfg(test)]
mod test {
    use super::super::gear::{Gear, GearType};
    use super::Player;

    // Creates A player (Not touching the DB)
    #[cfg(feature = "auto_save")]
    fn get_player() -> Player {
        // There is no reason for this to fail...
        Player::get(None, None).unwrap()
    }

    // Creates a player (Not touching the DB)
    #[cfg(not(feature = "auto_save"))]
    fn get_player() -> Player {
        Player::get()
    }

    // Test status
    #[test]
    fn status() {
        let mut player = get_player();
        player
            .status()
            .lines()
            .enumerate()
            .for_each(|(num, line)| match num {
                0 => assert_eq!(
                    line,
                    format!("Armor: {}/{}", Player::DEFENSE, Player::DEFENSE)
                ),
                1 => assert_eq!(
                    line,
                    format!("Health: {}/{}", Player::HEALTH, Player::HEALTH)
                ),
                2 => assert_eq!(line, "EXP: 0/10,000"),
                3 => assert_eq!(line, "Level: 0"),
                _ => unreachable!(),
            });
        player.damage_recieved = 10;
        player
            .status()
            .lines()
            .enumerate()
            .for_each(|(num, line)| match num {
                0 => assert_eq!(
                    line,
                    format!("Armor: {}/{}", Player::DEFENSE - 10, Player::DEFENSE)
                ),
                1 => assert_eq!(
                    line,
                    format!("Health: {}/{}", Player::HEALTH, Player::HEALTH)
                ),
                2 => assert_eq!(line, "EXP: 0/10,000"),
                3 => assert_eq!(line, "Level: 0"),
                _ => unreachable!(),
            });
    }

    // Test recieve_damage reduces the damage recieved
    #[test]
    fn recieve_damage() {
        #[allow(unused_mut)]
        let mut player = get_player();
        assert_eq!(player.current_health() as i64, Player::HEALTH as i64);
        panic!("Damage calculation isnt completed")
    }

    // Test Do Attack returns a number >= 0
    #[test]
    fn do_attack() {
        let mut player = get_player();
        let gear = Gear {
            accuracy: 0,
            attack: -200,
            defense: 0,
            enchant: 0,
            evasion: 0,
            name: String::new(),
            gear_type: GearType::Weapon,
            health: 0,
            is_boss: false,
        };
        player.gear.push(gear);
        panic!("Attack damage isnt completed");
    }

    // Test Stat changes by gear
    #[test]
    fn gear_modifers() {
        panic!("No written yet!")
    }
}
