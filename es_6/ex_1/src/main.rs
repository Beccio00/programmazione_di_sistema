mod primes;
//mod game;

fn main() {
    let limit = 1_000_000;
    println!("");

    for n_threads in 1..=16 {
        let start_1 = std::time::Instant::now();
        let primes_1 = primes::find_primes_counter(limit, n_threads);
        let duration_1 = start_1.elapsed();

        println!("Found {} primes in {:?} using {} threads in the first way", primes_1.len(), duration_1, n_threads);
        
        let start_2 = std::time::Instant::now();
        let primes_2 = primes::find_primes_mod(limit, n_threads);
        let duration_2 = start_2.elapsed();
        println!("Found {} primes in {:?} using {} threads in the second way", primes_2.len(), duration_2, n_threads);
        println!("");
    }    
    //let res = game::prepare("23423");
    //println!("{} {:?}", res.len(), res.iter().take(10).collect::<Vec<_>>());
}
