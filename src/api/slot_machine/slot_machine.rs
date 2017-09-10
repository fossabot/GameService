use rand::{thread_rng, Rng};
// Returns multiplier
pub fn slot_machine() -> (u64, Vec<String>) {
    let row: Vec<&'static str> = vec!["🍒", "🍊", "🍓", "🍍", "🍇", "🍉", "⭐"];
    //.iter_mut()
    //.map(|i| i.to_owned())
    //.collect();
    let mut choices: Vec<String> = Vec::with_capacity(3);
    let mut rng = thread_rng();
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    choices.push(row[rng.gen_range(0, row.len())].to_owned());
    let picks = choices.clone();
    choices.dedup();
    let mult: u64 = match choices.len() {
        3 => 0,
        2 => 1,
        _ => 2, // _ = 1
    };
    (mult, picks)
}
