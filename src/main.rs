#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate rand;
use std::collections::HashMap;


mod api;
use api::blackjack::BlackJackSessions;

#[cfg(test)]
mod tests;

fn main() {
    let ship = rocket::ignite().manage(BlackJackSessions::new(HashMap::with_capacity(100)));
    let ship = api::endpoints::router(ship);
    let _ = ship.launch();
}
