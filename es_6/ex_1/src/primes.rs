use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::thread;


pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes_counter(limit: u64, n_threads: u64) -> Vec<u64> {
    let count = Arc::new(Mutex::new(0));
    let primes = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    

    for _ in 0..n_threads{
        let count = Arc::clone(&count);
        let primes = Arc::clone(&primes);

        let handle = thread::spawn(move || {
            loop {
                let num = {
                    let mut count = count.lock().unwrap();
                    if *count > limit {
                        break;
                    }
                    let n = *count;
                    *count += 1;
                    n
                };
                
                if is_prime(num) {
                    let mut p = primes.lock().unwrap();
                    p.push(num);
                }
            };

        }); 

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    primes.lock().unwrap().clone()


}

pub fn find_primes_mod (limit: u64, n_threads: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut handles = vec![];

    for i in 0..n_threads {
        let handle = spawn(move || {
            let mut private_primes = vec![];
            let mut num = i;
            while num < limit {
                if is_prime(num) {
                    private_primes.push(num);
                }

                num += n_threads;
            }

            private_primes 
        });

        handles.push(handle);
    }

    for handle in handles {
        let mut private_primes = handle.join().unwrap();
        primes.append(&mut private_primes);
    }

    primes.sort();
    primes

}

