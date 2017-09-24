extern crate test;
use api::slot_machine::slot_machine;
use self::test::Bencher;
#[bench]
fn test_slot_machine(b: &mut Bencher) {
    b.iter(|| {
        let (mult, picks) = slot_machine();
        assert!([0f64, 1.5, 2f64].iter().any(|i| i == &mult));
        assert!(picks.len() <= 3);
    })
}
