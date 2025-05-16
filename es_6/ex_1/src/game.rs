
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use meval;


pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);
    
    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }   
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());

}

pub fn verify(v: &[String], n_thread: u64) -> Vec<String> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let chunk_size = (v.len() + n_thread as usize - 1) / n_thread as usize;
    let mut handles = Vec::new();

    for i in 0..n_thread {
        let results = Arc::clone(&results);
        let start = i as usize * chunk_size;
        let end = ((i as usize + 1)*chunk_size).min(v.len());
        let slice = v[start..end].to_vec();

        let handle = std::thread::spawn(move || {
            for string in slice {
                if let Ok(val) = meval::eval_str(string.clone()) {
                    if val == 10.0 && val.fract() == 0.0 {
                        let mut r = results.lock().unwrap();
                        r.push(string);
                    } 
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results.lock().unwrap().clone()

    
}