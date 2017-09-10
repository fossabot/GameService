// 5 Cards and you don't go over, you win
// Dealer wins on ties
// 21 is a win
// Start has 2 cards
// Player presses hit until they think they won
// Computer plays last

// TODO: Store the state

use api::blackjack::{Deck, Hand, Session};
use std::sync::Mutex;
use std::collections::HashMap;

type User = u64;
pub type BlackJackSessions = Mutex<HashMap<User, Session>>;


#[derive(Serialize, Deserialize)]
pub struct BlackJack {
    player: Hand,
    player_id: User,
    dealer: Hand,
    deck: Deck,
    bet: u64,
    first_turn: bool,
    player_stay_status: bool,
    dealer_stay_status: bool,
}


impl BlackJack {
    pub fn new(player_id: u64, bet: u64) -> Self {
        let mut deck = Deck::new();
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        player.add_card(deck.draw());
        player.add_card(deck.draw());
        dealer.add_card(deck.draw());
        dealer.add_card(deck.draw());

        BlackJack {
            player_id: player_id,
            player: player,
            dealer: dealer,
            deck: deck,
            bet: bet,
            first_turn: true,
            player_stay_status: false,
            dealer_stay_status: true,
        }
    }
    // Save will consume the BlackJack instance and return a session
    pub fn save(self) -> Session {
        Session::new(
            self.player,
            self.player_id,
            self.dealer,
            self.deck,
            self.bet,
            self.first_turn,
            self.player_stay_status,
            self.dealer_stay_status,
        )
    }
    pub fn restore(session: Session) -> Self {
        Self {
            player: Hand { cards: session.player },
            player_id: session.player_id,
            dealer: Hand { cards: session.dealer },
            deck: Deck { cards: session.deck },
            bet: session.bet,
            first_turn: session.first_turn,
            player_stay_status: session.player_stay,
            dealer_stay_status: session.dealer_stay,
        }
    }
    pub fn player_hit(&mut self) -> Result<(), &'static str> {
        match self.status() {
            0 => {
                if !self.player_stay_status {
                    Ok(self.player.add_card(self.deck.draw()))
                } else {
                    Err("You already pressed stay")
                }
            }
            1 => Err("You already lost"),
            2 => Err("You already won"),
            _ => Err("An unknown error has occurred"),
        }
    }
    pub fn player_stay(&mut self) {
        self.player_stay_status = true
    }
    fn dealer_hit(&mut self) -> Result<(), &'static str> {
        match self.status() {
            0 => {
                if !self.dealer_stay_status {
                    Ok(self.dealer.add_card(self.deck.draw()))
                } else {
                    Err("The Dealer already pressed stay")
                }
            }
            2 => Err("The dealer already lost"),
            1 => Err("The dealer already won"),
            _ => Err("An unknown error has occurred"),
        }

    }
    fn dealer_stay(&mut self) {
        self.dealer_stay_status = true
    }
    // 0 -> In progress
    // 1 -> Player lose
    // 2 -> Player win
    pub fn status(&self) -> u8 {
        if self.player_stay_status == false || self.dealer_stay_status == false {
            return 0;
        };
        let player_score = self.player.score();
        let dealer_score = self.dealer.score();
        if player_score == dealer_score {
            return 1;
        };
        if player_score > 21 {
            return 1;
        };
        if dealer_score > 21 {
            return 2;
        };

        if self.player.cards.len() == 5 {
            return 2;
        }
        if self.dealer.cards.len() == 5 {
            return 1;
        };
        0
    }

    // Computes dealer play
    pub fn dealer_play(&mut self) -> Result<(), &'static str> {
        if !self.player_stay_status {
            return Err("Player is not done yet");
        }
        while self.status() == 0 && self.dealer.score() < self.player.score() {
            self.dealer_hit().unwrap() // No errors should happen here
        }
        Ok(self.dealer_stay())
    }
}
