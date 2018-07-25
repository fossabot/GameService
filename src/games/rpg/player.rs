use super::errors;
#[cfg(feature = "auto_save")]
use super::errors::PlayerError;
use super::gear::{Gear, GearInfoStore};
#[cfg(feature = "auto_save")]
use diesel;
#[cfg(feature = "auto_save")]
use diesel::prelude::*;
use std::ops::Try;
use std::{fmt, i64};
#[cfg(feature = "auto_save")]
use ConnectionPool;

use failure::Error;

#[derive(Clone)]
pub struct Player {
    #[cfg(feature = "auto_save")]
    id: Option<u64>,
    pub exp: u64,
    pub damage_recieved: u32,
    pub gear: Vec<GearInfoStore>,
    #[cfg(feature = "auto_save")]
    conn_pool: Option<ConnectionPool>,
    #[cfg(feature = "auto_save")]
    save: bool,
}

impl Player {
    /// Health, this value is reduced when `DEFENSE` is 0
    const HEALTH: u32 = 100;
    /// Attack Stat, This stat effects attack damage
    const ATTACK: u32 = 10;
    /// Armor stat, 1:1 reduction of damage recieved
    const ARMOR: u32 = 100;
    /// Evasion stat, this stat effections your chance of dodging an attack (Not Implemented)
    const EVASION: u32 = 1;
    /// Accuracy stat, this stat effects your chance of hitting a target countering evasion `ACCURACY`:`EVASION` 3:1 (Not Implmented)
    const ACCURACY: u32 = 5;
    /// The required amount of EXP to level up
    const REQUIRED_EXP: u64 = 10_000;
    #[cfg(feature = "auto_save")]
    pub fn get(player_id: Option<u64>, pool: Option<&ConnectionPool>) -> Result<Self, Error> {
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
        let player_id = player_id.expect("Player ID where player ID should not be None");
        let pool = pool.into_result().or(Err(PlayerError::NoConnectionPool))?;

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
            damage_recieved: sess.damage_recieved as u32,
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
        self.exp / Self::REQUIRED_EXP
    }
    fn multiplyer(&self) -> f32 {
        self.level() as f32 / 80f32
    }
    fn base_health(&self) -> u32 {
        Self::HEALTH + (Self::HEALTH as f32 * self.multiplyer()).round() as u32
    }
    pub fn current_health(&self) -> u32 {
        let health: u32 = {
            self.gear
                .iter()
                .filter_map(|g| {
                    let gear: Result<Gear, errors::GearParseError> = (*g).into();
                    gear.ok()
                })
                .fold(self.base_health() as u32, |base, gear: Gear| {
                    let added_hp = gear.health();
                    if added_hp > 0 {
                        base.checked_add(added_hp as u32).unwrap_or(::std::u32::MAX)
                    } else {
                        base.checked_sub((added_hp.abs()) as u32).unwrap_or(0)
                    }
                })
        };
        let armor = self.armor();
        let recieved_damge = if self.damage_recieved < armor {
            0
        } else {
            self.damage_recieved.checked_sub(self.armor()).unwrap_or(0)
        };
        health.checked_sub(recieved_damge).unwrap_or(0)
    }
    pub fn accuracy(&self) -> u32 {
        let base = Self::ACCURACY + (Self::ACCURACY as f32 * self.multiplyer()).round() as u32;
        self.gear
            .iter()
            .filter_map(|g| {
                let g: Result<Gear, errors::GearParseError> = (*g).into();
                g.ok()
            })
            .fold(base, |base, gear| {
                let accuracy = gear.accuracy();
                if accuracy < 0 {
                    base.checked_sub((accuracy.abs()) as u32)
                        .unwrap_or(::std::u32::MIN)
                } else {
                    base.checked_add(accuracy as u32).unwrap_or(::std::u32::MAX)
                }
            })
    }
    pub fn evasion(&self) -> u32 {
        let base = Self::EVASION + (Self::EVASION as f32 * self.multiplyer()).round() as u32;
        self.gear
            .iter()
            .filter_map(|g| {
                let g: Result<Gear, errors::GearParseError> = (*g).into();
                g.ok()
            })
            .fold(base, |base, gear| {
                let evasion = gear.evasion();
                if evasion < 0 {
                    base.checked_sub((evasion.abs()) as u32)
                        .unwrap_or(::std::u32::MIN)
                } else {
                    base.checked_add(evasion as u32).unwrap_or(::std::u32::MAX)
                }
            })
    }
    pub fn attack(&self) -> u32 {
        let base = Self::ATTACK + (Self::ATTACK as f32 * self.multiplyer()).round() as u32;
        self.gear
            .iter()
            .filter_map(|g| {
                let g: Result<Gear, errors::GearParseError> = (*g).into();
                g.ok()
            })
            .fold(base, |base, gear| {
                let attack = gear.attack();
                if attack < 0 {
                    base.checked_sub((attack.abs()) as u32)
                        .unwrap_or(::std::u32::MIN)
                } else {
                    base.checked_add(attack as u32).unwrap_or(::std::u32::MAX)
                }
            })
    }
    pub fn armor(&self) -> u32 {
        let base = Self::ARMOR + (Self::ARMOR as f32 * self.multiplyer()).round() as u32;
        self.gear
            .iter()
            .filter_map(|g| {
                let g: Result<Gear, errors::GearParseError> = (*g).into();
                g.ok()
            })
            .fold(base, |base, gear| {
                let armor = gear.armor();
                if armor < 0 {
                    base.checked_sub((armor.abs()) as u32)
                        .unwrap_or(::std::u32::MIN)
                } else {
                    base.checked_add(armor as u32).unwrap_or(::std::u32::MAX)
                }
            })
    }
    pub fn resurect(&mut self) {
        self.damage_recieved = 0;
    }
    pub fn heal(&mut self, amount: u32) {
        self.damage_recieved -= amount;
    }
    pub fn is_alive(&self) -> bool {
        self.current_health() >= 1
    }
    pub fn do_attack(&self) -> u32 {
        self.attack()
    }
    /// Recieve Damage
    pub fn recieve_damage(&mut self, amount: u32) -> u32 {
        self.damage_recieved = self
            .damage_recieved
            .checked_add(amount)
            .unwrap_or(::std::u32::MAX);
        self.damage_recieved
    }
    #[cfg(feature = "auto_save")]
    pub fn save(&self) -> Result<(), Error> {
        use models::RPGSession;
        match self.conn_pool {
            Some(ref pool) => {
                let conn = pool.get()?;
                match self.id {
                    Some(id) => {
                        if !self.save {
                            return Err(PlayerError::DoNotSaveConfig.into());
                        }
                        let sess = RPGSession {
                            id: id as i64,
                            gear: c![g.to_string(), for g in &self.gear],
                            damage_recieved: i64::from(self.damage_recieved),
                            exp: self.exp as i64,
                        };
                        sess.save_changes::<RPGSession>(&*conn)?;
                        Ok(())
                    }
                    None => Err(PlayerError::NoID.into()),
                }
            }
            None => Err(PlayerError::NoConnectionPool.into()),
        }
    }
    pub fn status(&self) -> Status {
        let armor = self.armor();
        let new_armor = armor.checked_sub(self.damage_recieved).unwrap_or(0);
        let damage_recieved = (self.damage_recieved).checked_sub(armor).unwrap_or(0);
        let health = self.base_health();
        let new_health = health - damage_recieved;
        let level = self.level();
        let exp = self.exp - (level * 10_000);
        Status {
            def: (new_armor, armor),
            health: (new_health, health),
            level,
            exp,
            gear: self
                .gear
                .clone()
                .into_iter()
                .filter_map(|g| {
                    let g: Result<Gear, errors::GearParseError> = g.into();
                    g.ok()
                })
                .collect(),
            stats: Stats {
                attack: self.attack(),
                armor: self.armor(),
                evasion: self.evasion(),
                accuracy: self.accuracy(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Stats {
    attack: u32,
    armor: u32,
    evasion: u32,
    accuracy: u32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attack: {}\nArmor: {}\nEvasion: {}\nAccuracy: {}",
            pretty_num!(self.attack),
            pretty_num!(self.armor),
            pretty_num!(self.evasion),
            pretty_num!(self.accuracy)
        )
    }
}

#[derive(Debug, Serialize)]
pub struct Status {
    def: (u32, u32),
    health: (u32, u32),
    level: u64,
    exp: u64,
    gear: Vec<Gear>,
    stats: Stats,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut gear = self.gear.clone();
        gear.sort();
        let gear: String = if gear.is_empty() {
            String::from("None")
        } else {
            gear.into_iter().map(|g| g.pretty_text()).collect()
        };
        write!(
            f,
            "Armor: {}/{}\nHealth: {}/{}\nLevel: {} ({}/{})\n\nStats:\n{}\n\nGear:\n{}",
            pretty_num!(self.def.0),
            pretty_num!(self.def.1),
            pretty_num!(self.health.0),
            pretty_num!(self.health.1),
            pretty_num!(self.level),
            pretty_num!(self.exp),
            pretty_num!(Player::REQUIRED_EXP),
            self.stats,
            gear
        )
    }
}

#[cfg(test)]
mod test {
    use super::super::gear::GearInfoStore;
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
        let mut status = player.status();
        status
            .to_string()
            .lines()
            .enumerate()
            .for_each(|(num, line)| match num {
                0 => assert_eq!(line, format!("Armor: {}/{}", Player::ARMOR, Player::ARMOR)),
                1 => assert_eq!(
                    line,
                    format!("Health: {}/{}", Player::HEALTH, Player::HEALTH)
                ),
                2 => assert_eq!(line, "Level: 0 (0/10,000)"),
                3 | 9 => assert_eq!(line, ""),
                4 => assert_eq!(line, "Stats:"),
                5 => assert_eq!(line, format!("Attack: {}", Player::ATTACK)),
                6 => assert_eq!(line, format!("Armor: {}", Player::ARMOR)),
                7 => assert_eq!(line, format!("Evasion: {}", Player::EVASION)),
                8 => assert_eq!(line, format!("Accuracy: {}", Player::ACCURACY)),
                10 => assert_eq!(line, "Gear:"),
                11 => assert_eq!(line, "None"),
                _ => unreachable!(),
            });
        player.damage_recieved = 10;
        status = player.status();
        assert_eq!(status.def, ((Player::ARMOR - 10), Player::ARMOR));
        assert_eq!(status.health, (Player::HEALTH, Player::HEALTH));
        assert_eq!(status.exp, 0);
        assert_eq!(status.level, 0);
        assert!(status.gear.is_empty());
        let stats = status.stats;
        assert_eq!(stats.accuracy, Player::ACCURACY);
        assert_eq!(stats.attack, Player::ATTACK);
        assert_eq!(stats.armor, Player::ARMOR);
        assert_eq!(stats.evasion, Player::EVASION);
    }
    // Test recieve_damage reduces the damage recieved
    #[test]
    fn recieve_damage() {
        #[allow(unused_mut)]
        let mut player = get_player();
        assert_eq!(
            i64::from(player.current_health()),
            i64::from(Player::HEALTH)
        );
        panic!("Damage calculation isnt completed")
    }

    // Test Do Attack returns a number >= 0
    #[test]
    fn do_attack() {
        let mut player = get_player();
        player.gear.push(GearInfoStore::new(0, 0, 0));
        panic!("Attack damage isnt completed");
    }

    // Test Stat changes by gear
    #[test]
    fn gear_modifers() {
        panic!("No written yet!")
    }
}
