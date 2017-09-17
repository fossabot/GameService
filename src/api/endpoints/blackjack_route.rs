#![allow(needless_pass_by_value)]
use ConnectionPool;


use diesel::prelude::*;

use rocket::State;
use api::blackjack::{BlackJack, BlackJackResponse};
use rocket_contrib::Json;
use rocket_contrib::json::Json as JsonResp;
use serde_json::{to_value as to_json_value, Value as JsonValue};

type RouteResponseJson = JsonResp<JsonValue>;

#[get("/")]
fn active_sessions(db_pool: State<ConnectionPool>) -> RouteResponseJson {
    use schema::blackjack::dsl::*;
    let conn = db_pool.clone().get().unwrap();
    match blackjack
        .filter(status.is_null())
        .count()
        .get_result::<i64>(&*conn)
    {
        Ok(session_count) => Json(json!({"status_code": 200,
            "status": {
            "Ok": {
            "active_sessions": session_count}}
        })),
        Err(_) => Json(
            json!({"status_code": 500, "status":{"Err": "Failed to get active sessions"}}),
        ),
    }
}

#[get("/<user>/info")]
fn user_info(db_pool: State<ConnectionPool>, user: u64) -> RouteResponseJson {
    match BlackJack::restore(db_pool.clone(), user) {
        Ok(bj) => Json(to_json_value(BlackJackResponse::success(&bj)).unwrap()),
        Err(_) => Json(
            to_json_value(BlackJackResponse::error(501, "User does not exist")).unwrap(),
        ),
    }
}

#[post("/<user>/<bet>")]
fn create_user(db_pool: State<ConnectionPool>, user: u64, bet: u64) -> RouteResponseJson {
    match BlackJack::new(user, bet, db_pool.clone()) {
        Some(bj) => Json(to_json_value(BlackJackResponse::success(&bj)).unwrap()),
        None => Json(
            to_json_value(BlackJackResponse::error(
                501,
                "Failed to create user, user already exists.",
            )).unwrap(),
        ),
    }
}

#[post("/<user>/hit", rank = 2)]
fn player_hit(db_pool: State<ConnectionPool>, user: u64) -> RouteResponseJson {
    match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => match bj.player_hit() {
            Ok(_) => Json(to_json_value(BlackJackResponse::success(&bj)).unwrap()),
            Err(err) => Json(to_json_value(BlackJackResponse::error(501, err)).unwrap()),
        },
        Err(_) => Json(
            to_json_value(BlackJackResponse::error(501, "User does not exist")).unwrap(),
        ),
    }
}

#[post("/<user>/stay", rank = 2)]
fn player_stay(db_pool: State<ConnectionPool>, user: u64) -> RouteResponseJson {
    match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => {
            bj.player_stay();
            Json(to_json_value(BlackJackResponse::success(&bj)).unwrap())
        }
        Err(_) => Json(
            to_json_value(BlackJackResponse::error(501, "User does not exist")).unwrap(),
        ),
    }
}
