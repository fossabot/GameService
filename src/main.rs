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
use rocket::State;
use rocket_contrib::json::Json as JsonResp;

type User = u64;
type Bet = u64;
type BlackJackSessions = Mutex<HashMap<User, blackjack::BlackJackInstance>>;

#[get("/sessions")]
fn sessions(sessions: State<BlackJackSessions>) -> String {
    format!("Open Sessions: {}", sessions.lock().unwrap().len())
}

// Get a User's BlackJackInstance and return it as json 
#[get("/<user>")]
fn get_instance(sessions: State<BlackJackSessions>, user: User) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(sess) => {
            match sess.get(&user) {
                Some(s) => return rocket_contrib::Json(serde_json::to_value(s).unwrap()),
                None => {
                    rocket_contrib::Json(json!({"Error": "Failed to get user session"}))
                }
            }
        },
        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
    }

}

#[get("/<user>/new/<bet>")]
fn new_instance(sessions: State<BlackJackSessions>, user: User, bet: Bet) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(mut sess) => {
            // Get the session if it already exists, or make one
            let s = sess.entry(user).or_insert(blackjack::BlackJackInstance::new(user, bet));
            return rocket_contrib::Json(serde_json::to_value(s).unwrap())
        },
        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
    }
}

fn main() {
    let _ = rocket::ignite()
        .manage(BlackJackSessions::new(HashMap::with_capacity(100)))
        .mount("/games/blackjack", routes![sessions, get_instance, new_instance])
        .launch();
}
