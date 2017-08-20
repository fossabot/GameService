// Blackjack game handler
// TODO: Suspend inactive sessions
// TODO: Session Restore
use std::collections::HashMap;
use games::blackjack::BlackJackInstance;

pub struct BlackJack {
    // Hashmap<UserID, BlackJackSession>
    sessions: HashMap<u64, BlackJackInstance>
}

#[allow(dead_code)]
impl BlackJack {
    fn new() -> Self {
        BlackJack { sessions: HashMap::with_capacity(100) }
    }

    fn new_session(&mut self, user: u64, bet: u64) -> Result<(), ()> {
        self.sessions.insert(user, BlackJackInstance::new(user, bet));
        Ok(())
    }

}
