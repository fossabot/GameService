extern crate rand;
use games::blackjack::cards::{Card, create_deck};
use rand::Rng;
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BlackJackInstance {
    hand: Vec<Card>,
    bet: u32,
    comp_hand: Vec<Card>,
    user: u64, // User ID
    deck: Vec<Card>,
    user_stay: bool,
    comp_stay: bool,
}


#[allow(dead_code)]
impl BlackJackInstance {

    pub fn new(user: u64, bet: u32) -> BlackJackInstance {
        BlackJackInstance {
            user: user,
            bet: bet,
            hand: vec![],
            comp_hand: vec![],
            deck: create_deck(),
            comp_stay: false,
            user_stay: false

        }
    }

    pub fn draw(&mut self) {
        let deck_len = self.deck.len();
        let drawn_card = self.deck.remove(rand::thread_rng().gen_range(0, deck_len));
        self.hand.push(drawn_card);
    }

    pub fn comp_draw(&mut self) {
        let deck_len = self.deck.len();
        let drawn_card = self.deck.remove(rand::thread_rng().gen_range(0, deck_len));
        self.comp_hand.push(drawn_card);
    }

    /// Return the users current score
    pub fn score(&self) -> u16 {
        let mut aces = 0; 
        let mut score: u16 = 0;
        for card in &self.hand {
            if card.name == "ACE" {
                aces += 1;
                continue;
            }
            score += card.value;
        }
        if aces == 0 {return score}
        for _ace in 1..aces {
            if score > 10 {score+=1}
        }
        score
    }
    pub fn comp_score(&self) -> u16{
        let mut aces = 0; 
        let mut score: u16 = 0;
        for card in &self.comp_hand {
            if card.name == "ACE" {
                aces += 1;
                continue;
            }
            score += card.value;
        }
        if aces == 0 {return score}
        for _ace in 1..aces {
            if score > 10 {score+=1}
        }
        score
    }

}
