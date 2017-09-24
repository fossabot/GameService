use api::rps::rps as rps_game;
use rocket_contrib::Json;
use rocket_contrib::json::Json as JsonResp;
use serde_json::to_value;

#[get("/<weapon>/<bet>")]
fn rps(weapon: String, bet: u64) -> JsonResp{
    Json(to_value(rps_game(bet, &weapon)).unwrap())

}
