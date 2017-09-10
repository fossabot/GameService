use api::blackjack::{BlackJack, Card};


#[test]
fn test_blackjack_play() {
    let mut black_instance = BlackJack::new(0u64, 0u64);
    // Make sure the starting instance is in progress
    assert_eq!(black_instance.status(), 0);
    // Make sure that the dealer cannot play before player has pressed stay
    assert!(black_instance.dealer_play().is_err());
    // Make sure player's can press hit at start of game
    assert!(black_instance.player_hit().is_ok());
    // Set the player status to stay
    black_instance.player_stay();
    // Make sure the dealer can now play
    assert!(black_instance.dealer_play().is_ok());
    // Make sure the game ends after dealer play
    assert!([1u8, 2u8].iter().any(|s| &black_instance.status() == s));
    // Make sure player can not hit after they have stayed
    assert!(black_instance.player_hit().is_err());
    // Test that saving works
    let session = black_instance.save();
    // Test that session restore works
    let _ = BlackJack::restore(session).status();
}

#[test]
fn test_card_display() {
    assert_eq!(
        "Ace (1)".to_owned(),
        format!(
            "{}",
            Card {
                name: "Ace".to_owned(),
                value: 1,
            }
        )
    );
}
