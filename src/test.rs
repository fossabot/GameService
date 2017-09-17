use super::serde_json;

use rocket;
use rocket::local::Client;
use api::blackjack::BlackJackResponse;
use api::endpoints::router;

use establish_connection_pool;

#[derive(Deserialize)]
pub struct ActiveSessionsCount {
    active_sessions: u64,
}

#[derive(Deserialize)]
pub struct ActiveSessions {
    pub status_code: u16,
    pub status: Result<ActiveSessionsCount, String>,
}

fn create_client(use_db: bool) -> Client {
    if !use_db {
        Client::new(router(rocket::ignite())).unwrap()
    } else {
        Client::new(router(
            rocket::ignite().manage(establish_connection_pool().clone()),
        )).unwrap()
    }
}

#[test]
fn test_blackjack_routes() {
    let client = create_client(true);
    // Test session counter (should be 0)
    {
        let mut resp = client.get("/blackjack/").dispatch();
        let resp: ActiveSessions = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert_eq!(
            resp.status
                .expect("/blackjack/: Session count returned an error")
                .active_sessions,
            0
        );
    }
    // Test Creation and info route
    {
        let mut resp = client.post("/blackjack/0/0").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        let resp = resp.status
            .expect("An Error has occurred on session creation");
        assert_eq!(resp.dealer_hand.len(), 1);
        assert!(resp.game_state.is_none());
        let mut resp = client.get("/blackjack/0/info").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(
            resp.status
                .expect("An Error has occurred on session creation")
                .game_state
                .is_none()
        );
    }
    // Test Creation route fails
    {
        let mut resp = client.post("/blackjack/0/0").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 501);
    }
    // Test Hit Route
    {
        let mut resp = client.post("/blackjack/0/hit").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(match resp.status {
            Ok(stat) => stat.dealer_hand.len() >= 2,
            Err(_) => true,
        });
    }
    // Test Stay Route
    {
        let mut resp = client.post("/blackjack/0/stay").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(
            resp.status
                .expect("/blackjack/<user>/stay/: Failed on first stay.")
                .game_state
                .is_some()
        );
    }
    // Test Hit doesn't work
    {
        let mut resp = client.post("/blackjack/0/hit").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 501);
    }
    // Make sure new route works Now
    {
        let mut resp = client.post("/blackjack/0/0").dispatch();
        let resp: BlackJackResponse = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
    }
}

#[test]
fn test_slot_route() {
    let client = create_client(false);
    let mut resp = client.get("/slot_machine/23").dispatch();
    let resp: serde_json::Value = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    let ret: &f64 = &resp["return"].as_f64().unwrap();
    println!("{}", ret);
    assert!(vec![0f64, 34f64, 46f64].iter().any(|i| i == ret))
}
