use rand::{thread_rng, Rng};
// Returns multiplier
pub fn slot_machine() -> (f64, Vec<String>) {
    let row: Vec<&'static str> = vec!["🍒", "🍊", "🍓", "🍍", "🍇", "🍉", "⭐"];
    let mut choices: Vec<String> = Vec::with_capacity(3);
    let mut rng = thread_rng();
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    let picks = choices.clone();
    choices.dedup();
    let mult: f64 = match choices.len() {
        3 => 0f64,
        2 => 1.5,
        _ => 2f64, // _ = 1
    };
    (mult, picks)
}
