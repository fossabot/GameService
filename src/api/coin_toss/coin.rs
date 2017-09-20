#[cfg(test)]
extern crate test;
use rand::{thread_rng, Rng};
#[cfg(test)]
use self::test::Bencher;

const WEIGHT: u32 = 2;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Coin {
    Heads,
    Tails,
}

impl Coin {
    pub fn flip() -> Coin {
        let mut rng = thread_rng();
        if rng.gen_weighted_bool(WEIGHT) {
            Coin::Heads
        } else { Coin::Tails }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoinTossResult {
    status_code: u16,
    bet: u64,
    result: Result<Coin, String>
}

impl CoinTossResult {
    pub fn ok(bet: u64, side: Coin) -> Self{
        Self {
            status_code: 200,
            bet: bet,
            result: Ok(side)
        }
    }
    pub fn err(bet: u64, err: String) -> Self {
        Self {
            status_code: 501,
            bet: bet,
            result: Err(err)
        }
    }
}

pub fn guess_side(bet: u64, side: &str) -> CoinTossResult {
    let valid: bool =["heads", "h", "tails", "t"].iter().any(|&i| i == side.to_lowercase());
    if !valid { return CoinTossResult::err(bet, String::from("Not a valid side, heads/tails.")) };
    let guessed_side: Coin = if side.starts_with('h') { Coin::Heads } else { Coin::Tails };
    let side = Coin::flip();
    if guessed_side == side { CoinTossResult::ok(bet + (bet/2), side) } else { CoinTossResult::ok(0, side) }
}

#[bench]
fn bench_coin(bench: &mut Bencher) {
    bench.iter(|| {guess_side(0, "h")})
}
