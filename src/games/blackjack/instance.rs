extern crate rand;
use games::blackjack::cards::{Card, create_deck};
use rand::Rng;
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BlackJackInstance {
    hand: Vec<Card>,
    bet: u64,
    comp_hand: Vec<Card>,
    user: u64, // User ID
    deck: Vec<Card>,
    user_stay: bool,
    comp_stay: bool,
    complete: bool,
}


#[allow(dead_code)]
impl BlackJackInstance {

    pub fn new(user: u64, bet: u64) -> Self {
        BlackJackInstance {
            user: user,
            bet: bet,
            hand: vec![],
            comp_hand: vec![],
            deck: create_deck(),
            comp_stay: false,
            user_stay: false,
            complete: false,

        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        let deck_len = self.deck.len();
        if deck_len == 0 { // This shouldn't ever happen but saftey first
            return Err("The deck is empty!".to_owned()); // Handle this to calculate win /lose
        };
        let drawn_card = self.deck.remove(rand::thread_rng().gen_range(0, deck_len));
        self.hand.push(drawn_card);
        Ok(())
    }

    pub fn comp_draw(&mut self) -> Result<(), String> {
        let deck_len = self.deck.len();
        if deck_len == 0 { // This shouldn't ever happen but saftey first
            return Err("The deck is empty!".to_owned()); // Handle this to calculate win /lose
        }
        let drawn_card = self.deck.remove(rand::thread_rng().gen_range(0, deck_len));
        self.comp_hand.push(drawn_card);
        Ok(())
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

    /// Returns the current game status as a status code
    /// 1 - User Win
    /// 2 - User Loss
    /// 3 - Tie
    /// 4 - Last move
    /// 5 - In progress
    pub fn game_status(&mut self) -> u16 {
        let player_score = self.score();
        let comp_score = self.comp_score();
        // No more actions after 21
        if (player_score > 21) || (comp_score > 21) {
            self.complete = true;
            if player_score == comp_score {return 3};
            if player_score > comp_score {return 2};
            return 1
        };
        if self.comp_stay || self.user_stay {
            if !self.complete {return 4}; // to early to call it
            if player_score == comp_score {return 3};
        };
        return 5;

        
    }
    
    pub fn computer_play(&mut self) -> Result<(), String> {
        let game_status = self.game_status();
        if game_status <=3 {return Err("The game is already over".to_owned())};
        let player_score = self.score();
        let comp_score = self.comp_score();
        if comp_score == 21 {
            self.comp_stay = true; // Players can be stupid, by why should the AI be?
            return Ok(());
        };
        if player_score > 21 {
            self.comp_stay = true; // comp already one, no reason to waste computation
            return Ok(());
            // 20% chance to press stay
        }
        if rand::thread_rng().gen_range(0, 100) >= 80 {
                self.comp_stay = true;
                return Ok(());
        };

        self.comp_draw()

    }

}
