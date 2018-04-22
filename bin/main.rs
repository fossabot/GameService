#![plugin(rocket_codegen)]
#![feature(custom_derive, plugin, test)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
#![cfg_attr(test, allow(print_literal))]
#![cfg_attr(test, allow(needless_pass_by_value))]

extern crate diesel;
extern crate games_microservice;

#[cfg(feature = "web")]
extern crate rocket;
#[cfg(feature = "web")]
extern crate rocket_contrib;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

mod endpoints;

use games_microservice::{establish_connection_pool, games, ConnectionPool};
use rocket::Rocket;

#[cfg(feature = "web")]
pub fn create_rocket() -> Rocket {
    endpoints::router(rocket::ignite().manage(establish_connection_pool()))
}

fn main() {
    create_rocket().launch();
}
