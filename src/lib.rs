#![recursion_limit = "128"]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate rand;
extern crate r2d2;
extern crate r2d2_diesel;
#[allow(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;



use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;
pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;

mod api;

pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>;

pub mod schema;
pub mod models;
#[cfg(any(test))]
mod test;

pub fn establish_connection_pool() -> ConnectionPool {
    dotenv().ok();
    #[cfg(not(any(test, bench)))]
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    #[cfg(any(test, bench))]
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::new(r2d2::Config::default(), manager).expect("Failed to create pool");
    diesel::migrations::run_pending_migrations(&*(pool.clone().get().unwrap())).ok(); // Run migrations
    #[cfg(any(test, bench))]
    {
        use schema::blackjack::dsl::*;
        let conn = pool.clone().get().unwrap();
        let _num = diesel::delete(blackjack.filter(id.is_not_null()))
        .execute(&*conn)
        .expect("Error deleting posts");
    }
    pool
}

pub fn create_rocket() -> rocket::Rocket {
    api::endpoints::router(rocket::ignite().manage(establish_connection_pool()))
}
