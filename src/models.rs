use schema::blackjack;
#[derive(Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "blackjack"]
pub struct BJSession {
    pub id: i64,
    pub bet: Option<i64>, // None means it was claimed
    /*
    None - In progress
    true - Player Won
    false - Player Lost
    */
    pub status: Option<bool>,
    pub deck: Vec<String>,        // Empty when game ends
    pub player_hand: Vec<String>, // Empty when game ends
    pub dealer_hand: Vec<String>, // Empty when game ends
    pub player_stay: bool,        // False by default
    pub dealer_stay: bool,        // False by default
    pub first_turn: bool,         // True by default
}
