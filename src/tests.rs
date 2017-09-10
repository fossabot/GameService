#[allow(unused_imports)]
use super::{rocket, api, serde_json};
use api::blackjack::BlackJackSessions;
use std::collections::HashMap;

use rocket::local::Client;

// Creates test client
#[allow(dead_code)]
fn test_client() -> Client {
    let blackjack_sessions = BlackJackSessions::new(HashMap::with_capacity(1));
    let ship = rocket::ignite().manage(blackjack_sessions);
    let ship = api::endpoints::router(ship);
    Client::new(ship).unwrap()

}

// Because of the nature of blackjack routes, this has to be done as one major test
#[test]
fn test_blackjack_routes() {
    let client = test_client();

    // Make sure starting count is correct
    let mut resp = client.get("/blackjack/").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());
    assert_eq!(resp.get("active_sessions").unwrap(), 0);

    // Create a new user
    let mut resp = client.post("/blackjack/0/create/0").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());

    // Creating user twice should return an error if the user has not completed their game
    let mut resp = client.post("/blackjack/0/create/0").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_some());

    // Only 1 should exist
    let mut resp = client.get("/blackjack/").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());
    assert_eq!(resp.get("active_sessions").unwrap(), 1);

    // No Errors should occur on first hit
    let mut resp = client.post("/blackjack/0/hit").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());

    // Make sure no duplication happened
    let mut resp = client.get("/blackjack/").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());
    assert_eq!(resp.get("active_sessions").unwrap(), 1);

    // Stay No errors should occur (unless Mutex is poisoned)
    let mut resp = client.post("/blackjack/0/stay").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());

    // Make sure no duplication happened
    let mut resp = client.get("/blackjack/").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());
    assert_eq!(resp.get("active_sessions").unwrap(), 1);

    // Make sure you can create a new session after the previous is complete
    let mut resp = client.post("/blackjack/0/create/0").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert!(resp.get("Error").is_none());
}

#[test]
fn test_slots() {
    let client = test_client();
    let mut resp = client.get("/slot_machine/23").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    let ret: &u64 = &resp["return"].as_u64().unwrap();
    assert!(vec![0u64, 23u64, 46u64].iter().any(|i| i == ret))
}
