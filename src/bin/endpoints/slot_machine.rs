use api::slot_machine::slot_machine;
use rocket_contrib::json::Json as JsonResp;
use rocket_contrib::Json;
use serde_json::Value as JsonValue;

type RouteResponseJson = JsonResp<JsonValue>;
#[get("/<bet>")]
fn slots(bet: u64) -> RouteResponseJson {
    let (mult, picks) = slot_machine();
    let gain = (bet as f64 * mult).floor() as i64;
    Json(json!({
            "status_code": 200,
            "status": {"Ok": {
                "picks": picks,
                "bet": bet,
                "gain": gain
            }}
    }))
}
