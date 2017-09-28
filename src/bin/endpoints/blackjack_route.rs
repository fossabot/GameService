#![allow(needless_pass_by_value)]
use ConnectionPool;


use diesel::prelude::*;

use rocket::State;
use api::blackjack::{BlackJack, BlackJackResponse, SessionCount};
use rocket_contrib::Json;

#[get("/")]
fn active_sessions(db_pool: State<ConnectionPool>) -> Json<SessionCount> {
    use games_microservice::schema::blackjack::dsl::*;
    let conn = db_pool.clone().get().unwrap();
    Json(match blackjack
        .filter(status.is_null())
        .count()
        .get_result::<i64>(&*conn)
    {
        Ok(session_count) => SessionCount::count(session_count as u64),
        Err(_) => SessionCount::err("Failed to get active sessions"),
    })
}

#[get("/<user>")]
fn user_info(db_pool: State<ConnectionPool>, user: u64) -> Json<BlackJackResponse> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(bj) => BlackJackResponse::success(&bj),
        Err(_) => BlackJackResponse::error(501, "User does not exist"),
    })
}

#[post("/<user>/create/<bet>")]
fn create_user(db_pool: State<ConnectionPool>, user: u64, bet: u64) -> Json<BlackJackResponse> {
    Json(match BlackJack::new(user, bet, db_pool.clone()) {
        Some(bj) => BlackJackResponse::success(&bj),
        None => BlackJackResponse::error(
            501,
            "Failed to create, bet must be claimed before recreating a session.",
        ),
    })
}

#[post("/<user>/hit")]
fn player_hit(db_pool: State<ConnectionPool>, user: u64) -> Json<BlackJackResponse> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => match bj.player_hit() {
            Ok(_) => BlackJackResponse::success(&bj),
            Err(err) => BlackJackResponse::error(501, err),
        },
        Err(_) => BlackJackResponse::error(501, "User does not exist"),
    })
}

#[post("/<user>/stay")]
fn player_stay(db_pool: State<ConnectionPool>, user: u64) -> Json<BlackJackResponse> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => {
            bj.player_stay();
            BlackJackResponse::success(&bj)
        }
        Err(_) => BlackJackResponse::error(501, "User does not exist"),
    })
}
#[post("/<user>/claim")]
fn claim(db_pool: State<ConnectionPool>, user: u64) -> Json<BlackJackResponse> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(bj) => match bj.claim() {
            Ok(bj) => BlackJackResponse::success(&bj),
            Err(_) => BlackJackResponse::error(501, "Game is not over yet"),
        },
        Err(_) => BlackJackResponse::error(501, "User does not exist"),
    })
}
