
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;



use games::blackjack;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::sync::Mutex;
use rocket::State;
use rocket_contrib::json::Json as JsonResp;
type User = u64;
type Bet = u64;
type BlackJackSessions = Mutex<HashMap<User, blackjack::BlackJackInstance>>;

#[get("/sessions")]
pub fn sessions(sessions: State<BlackJackSessions>) -> String {
    format!("Open Sessions: {}", sessions.lock().unwrap().len())
}

// Get a User's BlackJackInstance and return it as json 
#[get("/<user>")]
pub fn get_instance(sessions: State<BlackJackSessions>, user: User) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(sess) => {
            match sess.get(&user) {
                Some(s) => return rocket_contrib::Json(serde_json::to_value(s).unwrap()),
                None => {
                    rocket_contrib::Json(json!({"Error": "Failed to get user session, does not exist.)"}))
                }
            }
        },
        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
    }

}

#[get("/<user>/new/<bet>")]
pub fn new_instance(sessions: State<BlackJackSessions>, user: User, bet: Bet) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(mut sess) => {
            // Get the session if it already exists, or make one
            sess.entry(user).or_insert(blackjack::BlackJackInstance::new(user, bet));
            if sess.get(&user).unwrap().complete {
                let _ = sess.remove(&user).is_none(); // Result doesnt matter
                let s = blackjack::BlackJackInstance::new(user, bet);
                sess.insert(user, s);
            };
            let s = sess.get(&user);
            return rocket_contrib::Json(serde_json::to_value(s).unwrap())
        },
        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
    }
}

#[get("/<user>/stay")]
pub fn user_stay(sessions: State<BlackJackSessions>, user: User) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.entry(user) {
                Occupied(mut s) => {
                    let e = s.get_mut();
                    if e.game_status() <= 3 {
                        return rocket_contrib::Json(json!({
                            "Error": "Game is already complete",
                            "status": e.game_status(),
                            "user_score":e.score(),
                            "comp_score":e.comp_score()
                        }))
                    };
                    if !e.comp_stay {e.computer_play().is_ok();};
                    e.complete = true;
                    return rocket_contrib::Json(json!({
                        "status": e.game_status(),
                        "user_score":e.score(),
                        "comp_score":e.comp_score()
                    }));
                },
                Vacant(_) => return rocket_contrib::Json(json!({"Error": "Failed to get user session, does not exist."})) 
            }
        },
        Err(_) => return rocket_contrib::Json(json!({"Erorr": "Faild to unlock sessions"}))
    }
}

#[get("/<user>/hit")]
pub fn user_hit(sessions: State<BlackJackSessions>, user: User) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.entry(user) {
                Occupied(mut s) => {
                    let e = s.get_mut();
                    if e.game_status() <= 3 {
                        return rocket_contrib::Json(json!({
                            "Error": "Game is already complete",
                            "status": e.game_status(),
                            "user_score":e.score(),
                            "comp_score":e.comp_score()
                        }))
                    };
                    match e.draw() {
                        Ok(_) => {
                            if e.comp_stay {e.complete = true}
                            else {e.computer_play().is_ok();};
                        },
                        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
                    }
                    return rocket_contrib::Json(json!({
                        "status": e.game_status(),
                        "user_score":e.score(),
                        "comp_score":e.comp_score()
                    }));
                },
                Vacant(_) => return rocket_contrib::Json(json!({"Error": "Failed to get user session, does not exist."})) 
            }
        },
        Err(_) => return rocket_contrib::Json(json!({"Erorr": "Faild to unlock sessions"}))
    }
}

#[get("/<user>/status")]
pub fn get_status(sessions: State<BlackJackSessions>, user: User) -> JsonResp<serde_json::Value> {
    match sessions.lock() {
        Ok(mut sess) => {
            match sess.entry(user) {
                Occupied(mut s) => {
                    let e = s.get_mut();
                    return rocket_contrib::Json(json!({
                        "status": e.game_status(),
                        "user_score": e.score(),
                        "comp_score": e.comp_score()
                    }));},
                Vacant(_) => return rocket_contrib::Json(json!({"Error": "Failed to get user session, does not exist."}))
            };
        },
        Err(e) => return rocket_contrib::Json(json!({"Error": format!("{:?}", e)}))
    };
}
