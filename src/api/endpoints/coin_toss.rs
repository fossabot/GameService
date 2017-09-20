use api::coin_toss::guess_side;
use rocket_contrib::Json;
use rocket_contrib::json::Json as JsonResp;
use serde_json::{to_value, Value as JsonValue};

type RouteResponseJson = JsonResp<JsonValue>;

#[allow(needless_pass_by_value)]
#[get("/<guess>/<bet>")]
fn coin_flip(guess: String, bet: u64) -> RouteResponseJson{
    Json(to_value(guess_side(bet, &guess)).unwrap())
}
