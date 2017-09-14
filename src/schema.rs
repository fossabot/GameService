table! {
    blackjack (id) {
        id -> Int8,
        bet -> Nullable<Int8>,
        status -> Nullable<Bool>,
        deck -> Array<Text>,
        player_hand -> Array<Text>,
        dealer_hand -> Array<Text>,
        player_stay -> Bool,
        dealer_stay -> Bool,
        first_turn -> Bool,
    }
}
