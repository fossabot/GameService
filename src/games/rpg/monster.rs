use rand::{self, Rng};
pub struct Monster {
    health: u64,
    pub attack: u64,
    pub defense: u64,
    pub accuracy: u64,
    pub evasion: u64,
    pub monster_type: MonsterType,
    floor: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MonsterType {
    Orc,
    Golem,
    Bat,
}

impl MonsterType {}

impl Monster {
    /// Returns a monster based on the floor
    pub fn new(monster: MonsterType, floor: u64) -> Self {
        use self::MonsterType::*;
        let mut rng = rand::thread_rng();
        match monster {
            // They dont want to die
            Orc => Self {
                health: Self::adjust_stats(floor, rng.gen_range(80, 120)),
                attack: Self::adjust_stats(floor, rng.gen_range(2, 5)),
                defense: Self::adjust_stats(floor, rng.gen_range(7, 9)),
                accuracy: Self::adjust_stats(floor, rng.gen_range(6, 10)),
                evasion: Self::adjust_stats(floor, rng.gen_range(2, 10)),
                monster_type: monster,
                floor,
            },
            // The nightmare of the dungeon
            Golem => Self {
                health: Self::adjust_stats(floor, rng.gen_range(120, 160)),
                attack: Self::adjust_stats(floor, rng.gen_range(7, 10)),
                defense: Self::adjust_stats(floor, rng.gen_range(10, 15)),
                accuracy: Self::adjust_stats(floor, rng.gen_range(7, 12)),
                evasion: Self::adjust_stats(floor, rng.gen_range(2, 4)),
                monster_type: monster,
                floor,
            },
            // Hard to hit, hit hard but weak health
            Bat => Self {
                health: Self::adjust_stats(floor, rng.gen_range(10, 40)),
                attack: Self::adjust_stats(floor, rng.gen_range(10, 15)),
                defense: Self::adjust_stats(floor, rng.gen_range(2, 10)),
                accuracy: Self::adjust_stats(floor, rng.gen_range(20, 30)),
                evasion: Self::adjust_stats(floor, rng.gen_range(3, 15)),
                monster_type: monster,
                floor,
            },
        }
    }
    fn adjust_stats(floor: u64, base: u64) -> u64 {
        base + (base as f64 * (floor as f64 / 3f64)) as u64
    }

    // Coins
    pub fn recieve_damage(&mut self, amount: u64) -> u64 {
        let damage = amount - (amount * (amount / self.defense));
        self.health = self.health.checked_sub(damage).unwrap_or(0);
        damage
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0u64
    }
    pub fn do_attack(&self) -> u64 {
        self.attack
    }
    pub fn loot(self) -> Option<u64> {
        if self.is_alive() {
            None
        } else {
            Some(Self::adjust_stats(
                self.floor + 5,
                rand::thread_rng().gen_range(0, 200),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn loot() {
        panic!("Not Written!")
    }

    #[test]
    fn recieve_damage() {
        panic!("Not Written!")
    }

    #[test]
    fn adjust_stats() {
        panic!("Not Written!")
    }

    #[test]
    fn is_alive() {
        panic!("Not Written!")
    }

}
