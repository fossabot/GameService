use rocket::Rocket;
mod blackjack_route;
mod slot_machine;

pub fn router(rocket: Rocket) -> Rocket {
    rocket
        .mount(
            "/blackjack",
            routes![
                blackjack_route::active_sessions,
                blackjack_route::user_info,
                blackjack_route::create_user,
                blackjack_route::player_hit,
                blackjack_route::player_stay,
            ],
        )
        .mount("/slot_machine", routes![slot_machine::slots])
}
