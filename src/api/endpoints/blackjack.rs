// TODO: https://github.com/Mikibot/GameService/blob/master/src/games/blackjack/routes.rs#L59-L149
use api::blackjack::{BlackJack, BlackJackSessions};

use rocket::State;
use rocket_contrib::json::Json as JsonResp;
use rocket_contrib::Json;
use serde_json::{Value as JsonValue, to_value as to_json_value};

type RouteResponseJson = Result<JsonResp<JsonValue>, JsonResp<JsonValue>>;

#[get("/")]
fn status(sessions: State<BlackJackSessions>) -> RouteResponseJson {
    match sessions.lock() {
        Ok(sess) => Ok(Json(json!({"active_sessions": sess.len()}))),
        Err(_) => Err(Json(json!({"Error": "An unknown error has occurred"}))),
    }
}
// TODO: Remove Unwraps and handle the errors =

#[get("/<user>")]
fn get_instance(sessions: State<BlackJackSessions>, user: u64) -> RouteResponseJson {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.remove(&user) {
                Some(s) => {
                    let bj = BlackJack::restore(s);
                    let resp = Json(to_json_value(&bj).unwrap());
                    sess.insert(user, bj.save());
                    Ok(resp)
                }
                None => Err(Json(json!({
                    "Error": "Failed to find user"
                }))),// DO A Thing
            }
        }
        Err(_) => Err(Json(json!({"Error": "An unknown error has occurred"}))),
    }
}


#[post("/<user>/create/<bet>")]
fn new_instance(sessions: State<BlackJackSessions>, user: u64, bet: u64) -> RouteResponseJson {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.remove(&user) {
                Some(s) => {
                    let bj = BlackJack::restore(s);
                    let resp = if bj.status() == 0 {
                        Err(Json(json!({"Error": "A game is already in progress"})))
                    } else {
                        let bj = BlackJack::new(user, bet);
                        Ok(Json(to_json_value(&bj).unwrap()))
                    };
                    sess.insert(user, bj.save());
                    resp
                }
                None => {
                    let bj = BlackJack::new(user, bet);
                    let resp = Json(to_json_value(&bj).unwrap());
                    sess.insert(user, bj.save());
                    Ok(resp)
                }// DO A Thing
            }
        }
        Err(_) => Err(Json(json!({"Error": "An unknown error has occurred"}))),
    }
}

#[post("/<user>/stay")]
fn user_stay(sessions: State<BlackJackSessions>, user: u64) -> RouteResponseJson {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.remove(&user) {
                Some(session) => {
                    let mut bj = BlackJack::restore(session);
                    bj.player_stay();
                    match bj.dealer_play() {
                        _ => (),
                    }; // Ignore errors
                    let resp = Json(to_json_value(&bj).unwrap());
                    sess.insert(user, bj.save());
                    Ok(resp)
                }
                None => Err(Json(json!({"Error": "Failed to find user"}))),
            }
        }
        Err(_) => Err(Json(json!({"Error": "An unknown error has occurred"}))),
    }
}

#[post("/<user>/hit")]
fn user_hit(sessions: State<BlackJackSessions>, user: u64) -> RouteResponseJson {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.remove(&user) {
                Some(session) => {
                    let mut bj = BlackJack::restore(session);
                    match bj.player_hit() {
                        Ok(_) => {
                            let resp = Json(to_json_value(&bj).unwrap());
                            sess.insert(user, bj.save());
                            Ok(resp)
                        }
                        Err(msg) => Err(Json(json!({"Error": msg}))),
                    }
                }
                None => Err(Json(json!({"Error": "Failed to find user"}))),
            }
        }
        Err(_) => Err(Json(json!({"Error": "An unknown error has occurred"}))),
    }
}
