use super::monster::{Monster, MonsterType};
use super::Player;
use rand::{self, Rng};
pub struct Dungeon {
    pub player: Player,
    pub current_floor: u64,
    pub balance: u64,
    auto_buy: bool,
    pub log: Vec<String>,
    gain_exp: bool,
}

impl Dungeon {
    /// new balance and message log
    pub fn challange(
        floor: u64,
        player: Player,
        balance: u64,
        auto_buy: bool,
    ) -> (u64, String, Player) {
        let mut current_floor = floor;
        while current_floor % 5 != 0 {
            current_floor -= 1;
        }

        let mut dungeon = Dungeon {
            current_floor,
            balance,
            auto_buy,
            log: vec![format!(
                "Starting on floor {}, AUtobuy: {}, Starting Bal: {}",
                floor, auto_buy, balance
            )],
            gain_exp: floor >= player.level() / 10,
            player,
        };
        dungeon.do_a_floor();
        dungeon.do_a_floor();
        dungeon.do_a_floor();
        dungeon.do_a_floor();
        dungeon.do_a_floor();
        if !dungeon.player.is_alive() {
            if dungeon.player.level() <= 10 {
                dungeon
                    .log
                    .push(String::from("Because you are lvl 10 or below res was free"));
            } else {
                dungeon.resurect();
            }
        }

        (dungeon.balance, dungeon.log.join("\n"), dungeon.player)
    }

    /// A Function to handle when the play was unable to handle the bull
    fn resurect_cant_afford_bill(&mut self) {
        self.log.push(String::from(
            "The priest was unable to find enough to pay the bill",
        ));
        let mut rng = rand::thread_rng();
        let gear = self.player.gear.clone();
        let mut gear_degraded = false;
        self.player.gear = gear.into_iter()
            .map(|mut g| {
                if g.enchant > 1 && rng.gen_weighted_bool(100) {
                    gear_degraded = true;
                    g.decrease_enchant(1);
                    self.log
                        .push(format!("The priest took an enchantment lvl of {}", g));
                }
                g
            })
            .collect();
        if !gear_degraded {
            self.log.push(String::from(
                "The merciful priest did not demand compensation",
            ));
        }
    }

    /// Handles the resurection of the player in the event of death
    fn resurect(&mut self) {
        if self.player.is_alive() {
            return;
        }
        if self.player.level() <= 10 {
            self.log.push(String::from(
                "Because you are lvl 10 or below your resurect was free and without cost",
            ));
            return;
        }
        let bill = self.player.level().pow(2) * 10;
        self.log.push(format!(
            "The priest searches your body to see if you can afford the resurection bill of {}",
            bill
        ));
        if self.balance > bill {
            self.resurect_cant_afford_bill();
        } else {
            self.balance -= bill;
            self.log.push(format!(
                "The priest took {} from you. Your new balance is: {}",
                bill, self.balance
            ));
        }
        self.player.exp = self.player.exp.checked_sub(100).unwrap_or(0);
        self.log.push(String::from("You lost 100 exp"));
    }

    /// Returns a random Potion effect heal/poison
    pub fn buy_potion(&mut self) {
        let price = 200 * (self.current_floor / 5) + 30;
        if self.balance < price {
            return;
        }
        self.balance -= price;
        let mut rng = rand::thread_rng();
        self.player.damage_recieved -= if !rng.gen_weighted_bool(30) {
            // Return Healing effect
            let effect = rng.gen_range(0i64, 100i64);
            self.log.push(format!(
                "Purchased a potion for {}, {} HP gained",
                price, effect
            ));
            effect
        } else {
            let effect = rng.gen_range(-50i64, 0i64);
            self.log.push(format!(
                "Purchased a potion for {} but it was rotten, {} HP Lost",
                price, effect
            ));
            effect
        };
    }
    /// Spawns a random monster, stats scaling to the floor
    pub fn spawn_monster(floor: u64) -> Monster {
        Monster::new(
            *rand::thread_rng()
                .choose(&[MonsterType::Bat, MonsterType::Golem, MonsterType::Orc])
                .unwrap_or(&MonsterType::Golem),
            floor,
        )
    }
    /// Code for executing a floor
    fn do_a_floor(&mut self) {
        if !self.player.is_alive() {
            return;
        }
        let mut rng = rand::thread_rng();
        let mut player_turn = rng.gen_weighted_bool(2);
        let mut monster = Dungeon::spawn_monster(self.current_floor);
        self.log
            .push(format!("Current Health: {}", self.player.current_health()));
        if player_turn {
            self.log.push(format!(
                "Encountered a {:?}, it hasnt Spotted you yet",
                monster.monster_type
            ))
        } else {
            self.log.push(format!(
                "{:?} has caught you by surprise",
                monster.monster_type
            ))
        }
        let monster_type = monster.monster_type;
        let mut monster_alive = monster.is_alive();
        let mut player_alive = self.player.is_alive();
        while monster_alive & player_alive {
            if player_turn {
                let player_health = self.player.current_health();
                player_turn = !player_turn;
                if self.auto_buy
                    && player_health < (player_health as f64 * (30f64 / 100f64)).round() as u64
                {
                    self.buy_potion()
                }
                if !self.player.is_alive() {
                    self.log.push(String::from("You have died"));
                }
                self.log.push(format!(
                    "You attacked {:?} for {}",
                    monster_type,
                    monster.recieve_damage(self.player.do_attack())
                ));
            } else {
                self.log.push(format!(
                    "{:?} Attacked you for {}",
                    monster_type,
                    self.player.recieve_damage(monster.do_attack())
                ));
            }
            player_alive = self.player.is_alive();
            monster_alive = monster.is_alive();
        }
        if player_alive {
            self.log
                .push(format!("Cleared floor {}", self.current_floor));
            if self.gain_exp {
                let lvl = self.player.level();
                self.player.exp += 10;
                if self.player.level() > lvl {
                    self.log
                        .push(String::from("Lvl'd up, Damage recieved reset"));
                    if self.player.damage_recieved > 0 {
                        self.player.damage_recieved = 0;
                    }
                }
            }
            self.current_floor += 1;
            let loot = monster.loot();
            match loot {
                Some(amount) => {
                    self.balance += amount;
                    self.log
                        .push(format!("Gained {} balance is now {}", amount, self.balance));
                }
                None => self.log.push(String::from("No loot dropped")),
            }
        }
    }
}
