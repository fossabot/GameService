use api::slot_machine::slot_machine;
#[test]
fn test_slot_machine() {
    let (mult, picks) = slot_machine();
    assert!([0u64, 1u64, 2u64].iter().any(|i| i == &mult));
    assert!(picks.len() <= 3);
}
