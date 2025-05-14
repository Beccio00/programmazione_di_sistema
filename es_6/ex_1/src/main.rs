mod primes;
//mod game;

fn main() {
    // let res = game::prepare("23423");
    // println!("{} {:?}", res.len(), res.iter().take(10).collect::<Vec<_>>());
    let res = primes::find_primes_counter(100, 5);
    println!("{} {:?}", res.len(), res.iter().collect::<Vec<_>>());
}
