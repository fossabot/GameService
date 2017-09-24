use api::rps::Weapons;
use api::rps::Response;
// Replace this with a response struct
pub fn rps(bet: u64, weapon: &str) -> Response {
    let weapon: Option<Weapons> = Weapons::from_str(weapon);
    if weapon.is_none() { return Response::error(bet, String::from("Valid choices are rock/paper/scissors"))};
    let weapon = weapon.unwrap();
    let comp = Weapons::rand_weapon();
    if weapon == comp { return Response::draw(bet, weapon.to_string(), comp.to_string()) };
    if weapon > comp { return Response::win(bet, weapon.to_string(), comp.to_string()) }; // Win
    Response::lose(weapon.to_string(), comp.to_string())
}

#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;
    use api::rps::rps;

    #[bench]
    fn bench_rps(b: &mut Bencher) {
        b.iter(|| rps(100, "r"))
    }
}
