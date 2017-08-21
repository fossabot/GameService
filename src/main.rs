#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod games;
use games::blackjack;
use std::collections::HashMap;
use std::sync::Mutex;

type User = u64;
type BlackJackSessions = Mutex<HashMap<User, blackjack::BlackJackInstance>>;


fn main() {
    let _ = rocket::ignite()
        .manage(BlackJackSessions::new(HashMap::with_capacity(100)))
        .mount("/games/blackjack", routes![blackjack::routes::sessions, blackjack::routes::get_instance, blackjack::routes::new_instance, blackjack::routes::user_hit, blackjack::routes::user_stay, blackjack::routes::get_status])
        .launch();
}
