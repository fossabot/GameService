use api::slot_machine::slot_machine;
use rocket_contrib::json::Json as JsonResp;
use rocket_contrib::Json;
use serde_json::Value as JsonValue;

type RouteResponseJson = JsonResp<JsonValue>;
#[get("/<bet>")]
fn slots(bet: u64) -> RouteResponseJson {
    let (mult, picks) = slot_machine();
    Json(
        json!({"picks": picks, "return": (bet as f64 * mult).floor()}),
    )
}
