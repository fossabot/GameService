use rocket::Rocket;

mod blackjack;
mod slot_machine;


pub fn router(rocket: Rocket) -> Rocket {
    rocket
        .mount(
            "/blackjack",
            routes![
            blackjack::status,
            blackjack::get_instance,
            blackjack::new_instance,
            blackjack::user_stay,
            blackjack::user_hit,
        ],
        )
        .mount("/slot_machine", routes![slot_machine::slots])
}
