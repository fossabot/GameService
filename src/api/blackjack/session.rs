use api::blackjack::{Hand, Card, Deck};
pub struct Session {
    pub player: Vec<Card>,
    pub dealer: Vec<Card>,
    pub deck: Vec<Card>,
    pub player_id: u64,
    pub bet: u64,
    pub first_turn: bool,
    pub player_stay: bool,
    pub dealer_stay: bool,
}

impl Session {
    pub fn new(
        player: Hand,
        player_id: u64,
        dealer: Hand,
        deck: Deck,
        bet: u64,
        firt_turn: bool,
        player_stay: bool,
        dealer_stay: bool,
    ) -> Self {
        Self {
            player_id: player_id,
            player: player.cards,
            dealer: dealer.cards,
            deck: deck.cards,
            bet: bet,
            first_turn: firt_turn,
            player_stay: player_stay,
            dealer_stay: dealer_stay,
        }
    }
}
