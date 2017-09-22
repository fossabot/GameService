use rand::{thread_rng, Rng};
// Returns multiplier
const ROW: [&'static str; 7] = ["🍒", "🍊", "🍓", "🍍", "🍇", "🍉", "⭐"];
pub fn slot_machine() -> (f64, Vec<&'static str>) {
    let mut choices: Vec<&'static str> = Vec::with_capacity(3);
    let mut rng = thread_rng();
    let row_len = ROW.len();
    choices.push(ROW[rng.gen_range(0, row_len)]);
    choices.push(ROW[rng.gen_range(0, row_len)]);
    choices.push(ROW[rng.gen_range(0, row_len)]);
    let picks = choices.clone();
    choices.sort();
    choices.dedup();
    let mult: f64 = match choices.len() {
        3 => 0f64,
        2 => 1.5,
        _ => 2f64, // _ = 1
    };
    (mult, picks)
}


#[cfg(any(test, bench))]
mod tests;
