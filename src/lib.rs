#![recursion_limit = "128"]
#![feature(custom_derive, test, const_fn)]
#![allow(unknown_lints)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
#[macro_use]
extern crate serde_derive;



use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;
pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;

pub mod api;

pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>;

pub mod schema;
pub mod models;


pub fn establish_connection_pool() -> ConnectionPool {
    dotenv().ok();

    #[cfg(not(any(test, bench)))]
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    #[cfg(any(test, bench))]
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::new(
        match env::var("MAX_POOL") {
            Ok(size) => r2d2::Config::builder()
                .pool_size(size.parse::<u32>().unwrap())
                .build(),
            Err(_) => r2d2::Config::builder().pool_size(10).build(),
        },
        manager,
    ).expect("Failed to create pool");
    // Run migrations
    diesel::migrations::run_pending_migrations(&*(pool.clone().get().unwrap())).ok();
    #[cfg(any(test, bench))]
    {
        use schema::blackjack::dsl::*;
        let conn = pool.clone().get().unwrap();
        let _num = diesel::delete(blackjack.filter(id.is_not_null()))
            .execute(&*conn)
            .expect("Error deleting Previous BlackJack Test data");
    }
    pool
}
