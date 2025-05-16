mod primes;
mod game;

fn main() {
    let limit = 1_000_000;
    println!("");
    let s = "23423";
    let ops = game::mk_ops(&['+', '-', '*', '/'], 4);
    let res = game::prepare(s);
    

    for n_threads in 1..=16 {
        let start = std::time::Instant::now();
        let primes_1 = primes::find_primes_counter(limit, n_threads);
        let duration = start.elapsed();

        println!("Found {} primes in {:?} using {} threads in the first way", primes_1.len(), duration, n_threads);
        
        let start = std::time::Instant::now();
        let primes_2 = primes::find_primes_mod(limit, n_threads);
        let duration = start.elapsed();
        println!("Found {} primes in {:?} using {} threads in the second way", primes_2.len(), duration, n_threads);
        println!("");
        
    }

    for n_threads in 1..=16 {
        let start = std::time::Instant::now();
        let res = game::verify(&res, n_threads);
        let duration = start.elapsed();
        println!("Found {} results in {:?} using {} threads: {:?}", res.len(), duration, n_threads, res.iter().take(20).collect::<Vec<_>>());
        println!();
    }
 
    

    
}
