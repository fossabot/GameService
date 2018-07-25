#[cfg(feature = "auto_save")]
use super::Session;
use super::{CardParseError, Deck, DeckError, Hand};
#[cfg(feature = "auto_save")]
use diesel;
#[cfg(feature = "auto_save")]
use diesel::prelude::*;
#[cfg(feature = "auto_save")]
use diesel::result::Error as DieselResultError;
use failure::Error;
#[cfg(feature = "auto_save")]
use r2d2::Error as R2d2Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
#[cfg(feature = "auto_save")]
use ConnectionPool;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum GameState {
    InProgress,
    PlayerWon,
    PlayerLost,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(match *self {
            GameState::InProgress => "In Progress",
            GameState::PlayerLost => "Dealer Won",
            GameState::PlayerWon => "Player Won",
        })
    }
}

#[derive(Debug, Fail)]
pub enum BlackJackError {
    #[fail(display = "The dealer has already lost this game.")]
    DealerAlreadyLost,
    #[fail(
        display = "The dealer has already chosen to stay, the dealer is not allowed to make another move."
    )]
    DealerAlreadyPressedStay,
    #[fail(display = "The dealer has already won this game.")]
    DealerAlreadyWon,
    #[fail(display = "The game is over.")]
    GameOver,
    #[fail(
        display = "Invalid result count, found {} and expected {}",
        ct,
        expt
    )]
    InvalidResultCount { ct: usize, expt: usize },
    #[fail(display = "No Card")]
    NoCard,
    #[fail(display = "Player has already lost this game.")]
    PlayerAlreadyLost,
    #[fail(
        display = "Player has already chosen to stay, the play is not premitted to make another move"
    )]
    PlayerAlreadyPressedStay,
    #[fail(display = "Player has already won this game.")]
    PlayerAlreadyWon,
    #[fail(
        display = "Player has not finished making their moves, the dealer is not premitted to take action."
    )]
    PlayerNotDoneYet,
    #[fail(display = "This session already exists")]
    SessionAlreadyExists,
    #[fail(display = "Game is still in progress")]
    GameStillInProgress,
    #[fail(display = "Session does not exist")]
    SessionDoesNotExist,
    #[fail(display = "Error Parsing Card")]
    CardParseError(#[cause] CardParseError),
    #[cfg(feature = "auto_save")]
    #[fail(display = "Database Error(ORM)")]
    DieselResultError(#[cause] DieselResultError),
    #[cfg(feature = "auto_save")]
    #[fail(display = "Database Error (Connection)")]
    R2d2Error(#[cause] R2d2Error),
}

impl From<CardParseError> for BlackJackError {
    fn from(err: CardParseError) -> Self {
        BlackJackError::CardParseError(err)
    }
}
#[cfg(feature = "auto_save")]
impl From<DieselResultError> for BlackJackError {
    fn from(err: DieselResultError) -> Self {
        BlackJackError::DieselResultError(err)
    }
}
#[cfg(feature = "auto_save")]
impl From<R2d2Error> for BlackJackError {
    fn from(err: R2d2Error) -> Self {
        BlackJackError::R2d2Error(err)
    }
}

impl From<DeckError> for BlackJackError {
    fn from(_: DeckError) -> Self {
        BlackJackError::NoCard
    }
}

impl BlackJackError {
    /// Return Status Code based on the error
    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    pub fn status_code(&self) -> u16 {
        use self::BlackJackError::*;
        match *self {
            // CardParse(_) => 500,
            DealerAlreadyLost => 501,
            DealerAlreadyPressedStay => 500,
            DealerAlreadyWon => 501,
            NoCard => 500,
            PlayerAlreadyLost => 501,
            PlayerAlreadyPressedStay => 500,
            GameOver => 501,
            InvalidResultCount { .. } => 500,
            PlayerAlreadyWon => 501,
            PlayerNotDoneYet => 501,
            SessionAlreadyExists => 501,
            GameStillInProgress => 501,
            SessionDoesNotExist => 501,
            #[cfg(feature = "auto_save")]
            _ => 500,
        }
    }
}

// TODO: Implement Surrender
// TODO: Implement Insurrence

#[derive(Clone)]
pub struct BlackJack {
    pub player: Hand,
    #[cfg(feature = "auto_save")]
    pub player_id: u64,
    pub dealer: Hand,
    deck: Deck,
    pub bet: u64,
    // Used for responses
    pub first_turn: bool,
    pub player_stay_status: bool,
    pub dealer_stay_status: bool,
    pub gain: i64,
    #[cfg(feature = "auto_save")]
    db_pool: ConnectionPool,
    #[cfg(feature = "auto_save")]
    claimed: bool,
}

impl BlackJack {
    #[cfg(feature = "auto_save")]
    pub fn new(
        player_id: u64,
        new_bet: u64,
        db_pool: ConnectionPool,
    ) -> Result<Self, BlackJackError> {
        use schema::blackjack as blackjack_schema;
        use schema::blackjack::dsl::*;

        // TODO: Make this safer (low)
        let conn = db_pool.get()?;
        let num: i64 = blackjack
            .filter(id.eq(player_id as i64))
            .count()
            .get_result(&*conn)
            // TODO: Make this safer
            .unwrap_or_default();

        if num != 0 {
            return Err(BlackJackError::SessionAlreadyExists);
        }

        let mut new_deck = Deck::new();
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        player.add_card(new_deck.draw()?);
        player.add_card(new_deck.draw()?);
        dealer.add_card(new_deck.draw()?);
        dealer.add_card(new_deck.draw()?);

        let num: i64 = blackjack
            .filter(id.eq(player_id as i64))
            .count()
            .get_result(&*conn)
            // TODO: Make this safer
            .unwrap_or_default();

        if num == 0 {
            let sess = Session {
                id: player_id as i64,
                bet: Some(new_bet as i64),
                dealer_hand: dealer.export().1,
                dealer_stay: false,
                deck: new_deck.export(),
                first_turn: true,
                player_hand: player.export().1,
                player_stay: false,
                status: None,
            };

            let _: Session = diesel::insert_into(blackjack_schema::table)
                .values(&sess)
                .get_result(&*conn)
                .expect("Error saving sessions");
        }

        Ok(Self {
            player_id,
            player,
            dealer,
            deck: new_deck,
            bet: new_bet,
            first_turn: true,
            player_stay_status: false,
            dealer_stay_status: false,
            db_pool,
            claimed: false,
            gain: 0i64,
        })
    }
    #[cfg(not(feature = "auto_save"))]
    pub fn new(bet: u64) -> Result<Self, BlackJackError> {
        let mut deck = Deck::new();
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        player.add_card(deck.draw().cause("Failed to draw card from deck"));
        player.add_card(deck.draw().cause("Failed to draw card from deck"));
        dealer.add_card(deck.draw().cause("Failed to draw card from deck"));
        dealer.add_card(deck.draw().cause("Failed to draw card from deck"));
        Ok(Self {
            deck,
            player,
            dealer,
            bet,
            dealer_stay_status: false,
            player_stay_status: false,
            first_turn: true,
            gain: 0i64,
        })
    }

    #[cfg(feature = "auto_save")]
    pub fn restore(db_pool: &ConnectionPool, player: u64) -> Result<Self, Error> {
        use schema::blackjack::dsl::*;

        // TODO: Make this safer (low)
        let conn = db_pool.get()?;
        let results = blackjack
            .filter(id.eq(player as i64))
            .limit(1)
            .load::<Session>(&*conn)
            // TODO: make this safer
            .unwrap_or_default();

        let len = results.len();

        if len != 1 {
            // There should be one result if not, nothing is found
            return Err(BlackJackError::InvalidResultCount { ct: len, expt: 1 }.into());
        }

        // nb: indicing 0 is safe, length already checked
        let session: &Session = &results[0];

        if session.bet.is_none() {
            // Game is over, Start a new one
            return Err(BlackJackError::GameOver.into());
        }

        let player_bet = session.bet.unwrap();

        Ok(Self {
            player: Hand {
                cards: c![card.parse()?, for card in &session.player_hand],
            },
            player_id: session.id as u64,
            dealer: Hand {
                cards: c![card.parse()?, for card in &session.dealer_hand],
            },
            deck: Deck {
                cards: c![card.parse()?, for card in &session.deck],
            },
            bet: player_bet as u64,
            player_stay_status: session.player_stay,
            dealer_stay_status: session.dealer_stay,
            first_turn: session.first_turn,
            db_pool: db_pool.clone(),
            claimed: false,
            gain: 0i64,
        })
    }

    pub fn player_hit(&mut self) -> Result<(), Error> {
        match self.status() {
            GameState::InProgress => if !self.player_stay_status {
                self.first_turn = false;
                self.player.add_card(self.deck.draw()?);
                Ok(())
            } else {
                Err(BlackJackError::PlayerAlreadyPressedStay.into())
            },
            GameState::PlayerLost => Err(BlackJackError::DealerAlreadyLost.into()),
            GameState::PlayerWon => Err(BlackJackError::DealerAlreadyWon.into()),
        }
    }

    pub fn player_stay(&mut self) -> Result<(), Error> {
        if !self.player_stay_status {
            self.player_stay_status = true;

            self.dealer_play()?;
        }

        Ok(())
    }

    fn dealer_hit(&mut self) -> Result<(), Error> {
        self.first_turn = false;
        match self.status() {
            GameState::InProgress => if !self.dealer_stay_status {
                self.dealer.add_card(self.deck.draw()?);
                Ok(())
            } else {
                Err(BlackJackError::DealerAlreadyPressedStay.into())
            },
            GameState::PlayerWon => Err(BlackJackError::DealerAlreadyLost.into()),
            GameState::PlayerLost => Err(BlackJackError::DealerAlreadyWon.into()),
        }
    }

    fn dealer_stay(&mut self) {
        self.dealer_stay_status = true
    }

    pub fn status(&self) -> GameState {
        let player_score = self.player.score();
        let dealer_score = self.dealer.score();

        if self.player.cards.len() == 5 && player_score <= 21 {
            return GameState::PlayerWon;
        }

        if self.dealer.cards.len() == 5 && dealer_score <= 21 {
            return GameState::PlayerWon;
        }

        if player_score == 21 {
            return GameState::PlayerWon;
        }

        if dealer_score == 21 {
            return GameState::PlayerLost;
        }

        if !(self.player_stay_status || self.dealer_stay_status) {
            return GameState::InProgress;
        }

        if player_score == dealer_score {
            return GameState::PlayerLost;
        }

        if player_score > 21 {
            return GameState::PlayerLost;
        }

        if dealer_score > 21 {
            return GameState::PlayerWon;
        }

        if player_score > dealer_score {
            return GameState::PlayerWon;
        }

        if player_score < dealer_score {
            return GameState::PlayerLost;
        }

        GameState::InProgress
    }

    // Computes dealer play
    pub fn dealer_play(&mut self) -> Result<(), Error> {
        if !self.player_stay_status {
            return Err(BlackJackError::PlayerNotDoneYet.into());
        }

        self.first_turn = false;

        while self.status() == GameState::InProgress && self.dealer.score() <= 17 {
            self.dealer_hit()?; // No errors should happen here
        }

        self.dealer_stay();

        Ok(())
    }
    #[cfg(feature = "auto_save")]
    pub fn save(&self) -> Result<(), BlackJackError> {
        let conn = self.db_pool.get()?;

        let (game_status, bet): (Option<bool>, Option<i64>) = match self.status() {
            GameState::InProgress => (None, Some(self.bet as i64)),
            GameState::PlayerWon => (Some(true), None),
            GameState::PlayerLost => (Some(false), None),
        };

        let sess = Session {
            id: self.player_id as i64,
            bet,
            dealer_hand: self.dealer.export().1,
            dealer_stay: self.dealer_stay_status,
            deck: self.deck.export(),
            first_turn: self.first_turn,
            player_hand: self.player.export().1,
            player_stay: self.player_stay_status,
            status: game_status,
        };

        let _: Session = sess.save_changes(&*conn)?;

        Ok(())
    }
    #[cfg(not(feature = "auto_save"))]
    pub fn save(&self) -> Result<(), BlackJackError> {
        unimplemented!()
    }
    #[cfg(feature = "auto_save")]
    fn db_remove(&self) -> Result<(), Error> {
        use schema::blackjack::dsl::*;

        let conn = self.db_pool.get()?;

        diesel::delete(blackjack.filter(id.eq(self.player_id as i64))).execute(&*conn)?;

        Ok(())
    }

    /// Consumes session and returns Gain
    pub fn claim(&mut self) -> Result<i64, BlackJackError> {
        match self.status() {
            GameState::InProgress => Err(BlackJackError::GameStillInProgress),
            GameState::PlayerLost => {
                #[cfg(feature = "auto_save")]
                {
                    self.claimed = true;
                }
                self.gain = -(self.bet as i64);

                Ok(self.gain)
            }
            GameState::PlayerWon => {
                #[cfg(feature = "auto_save")]
                {
                    self.claimed = true;
                }
                self.gain = self.bet as i64;

                Ok(self.gain)
            }
        }
    }
}

#[cfg(feature = "auto_save")]
impl Drop for BlackJack {
    fn drop(&mut self) {
        if !self.claimed {
            // Save before vanishing

            if let Err(why) = self.save() {
                panic!("Error saving to DB: {:?}", why);
            }
        } else if let Err(why) = self.db_remove() {
            panic!("Error removing from DB: {:?}", why);
        }
    }
}
