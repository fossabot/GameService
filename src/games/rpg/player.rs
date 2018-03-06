use super::Gear;
#[cfg(feature = "auto_save")]
use diesel::prelude::*;
#[cfg(feature = "auto_save")]
use diesel;
#[cfg(feature = "auto_save")]
use ConnectionPool;
use super::errors::PlayerError;

#[derive(Clone)]
pub struct Player {
    #[cfg(feature = "auto_save")]
    id: u64,
    pub exp: u64,
    pub damage_recieved: i64,
    pub gear: Vec<Gear>,
    #[cfg(feature = "auto_save")]
    conn_pool: ConnectionPool,
    #[cfg(feature = "auto_save")]
    save: bool,
}

impl Player {
    const HEALTH: u64 = 100;
    const ATTACK: i64 = 10;
    const DEFENSE: i64 = 5;
    const EVASION: i64 = 1;
    const ACCURACY: i64 = 5;
    #[cfg(feature = "auto_save")]
    pub fn get(player_id: u64, pool: &ConnectionPool) -> Result<Self, PlayerError> {
        use schema::rpgplayer::dsl::*;
        use schema::rpgplayer as rpgplayer_schema;
        use models::RPGSession;
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
            id: sess.id as u64,
            exp: sess.exp as u64,
            damage_recieved: sess.damage_recieved,
            gear: c![g.parse()?, for g in sess.gear],
            conn_pool: pool.clone(),
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
        (Self::HEALTH as f64 * self.multiplyer()) as u64
    }
    pub fn current_health(&self) -> u64 {
        let health: i64 = self.gear
            .iter()
            .fold(self.base_health() as i64, |base, gear| {
                base + gear.health()
            });
        if health < 0 || self.damage_recieved as i64 > health {
            0
        } else {
            (health - self.damage_recieved) as u64
        }
    }
    pub fn accuracy(&self) -> i64 {
        let base = (Self::ACCURACY as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| base + gear.accuracy())
    }
    pub fn evasaion(&self) -> i64 {
        let base = (Self::EVASION as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| base + gear.evasion())
    }
    pub fn attack(&self) -> i64 {
        let base = (Self::ATTACK as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| gear.attack() + base)
    }
    pub fn defense(&self) -> i64 {
        let base = (Self::DEFENSE as f64 * self.multiplyer()) as i64;
        self.gear
            .iter()
            .fold(base, |base, gear| base + gear.defense()) as i64
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
    pub fn recieve_damage(&mut self, amount: u64) -> u64 {
        let damage = amount as i64 - (amount as i64 * (amount as i64 / self.defense()));
        if damage <= 0 {
            return 0;
        }
        self.damage_recieved = self.damage_recieved.checked_add(damage).unwrap_or(0);
        damage as u64
    }
    #[cfg(feature = "auto_save")]
    pub fn save(&self) -> Result<(), PlayerError> {
        use models::RPGSession;
        let conn = self.conn_pool.get()?;
        let sess = RPGSession {
            id: self.id as i64,
            gear: c![g.to_string(), for g in &self.gear],
            damage_recieved: self.damage_recieved,
            exp: self.exp as i64,
        };
        sess.save_changes::<RPGSession>(&*conn)?;
        Ok(())
    }
}
