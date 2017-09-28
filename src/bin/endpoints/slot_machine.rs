use api::slot_machine::{SlotMachine, Response};
use rocket_contrib::Json;

#[get("/<bet>")]
fn slots(bet: u64) -> Json<Response> {
    Json(Response {
        status_code: 200,
        status: Ok(SlotMachine::new(bet))
    })
}
