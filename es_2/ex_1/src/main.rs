use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::ptr::read;

fn stats(text: &str) -> [u32; 26] {
    let mut count = [0; 26];
    for c in text.chars(){
        let c1 = c.to_ascii_lowercase();
        //c.is_ascii_alphabetic();
        let index = c1 as u32 - 'a' as u32;
            if (index >= 0 && index < 26) {
                count[index as usize] += 1;
            }
    }
    count 
}

fn is_pangram(counts: &[u32]) -> bool {

    if counts.len() != 26 {
        return false;
    }

    for c in counts {
        if *c == 0 {
            return false;
        }
    }
    true
 }

//Questo lo vede come un indirizzo di memoria
/* fn f1(v: &[u32]) {  for el in v {
        println!("{}", el);
 } } 

Questo lo vede come un intero perché il puntatore è un intero
 fn f2(v: &[u32]) { for &el in v {
        println!("{}", el);
 } }*/
    

// call this function from main
// load here the contents of the file

pub fn run_pangram() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Usage: pangram -- <filename>");
        return;
    }

    let file = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line.expect("Errore nella lettura"));
    }

    let counts = stats(&contents);
    let pangram = is_pangram(&counts);
    //f1(&counts);
    //f2(&counts);
    println!("\"{}\" is {} a pangram", contents.trim(), if pangram { "" } else { "not" });

    for (i, &count) in counts.iter().enumerate() { 
        println!("{} {}", (b'a' + i as u8) as char, count);
    }


}


// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{   
    // tests are separated modules, yuou must import the code you are testing
    use super::*;
    
    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }
    
    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }    
    
    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test] 
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
            
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}

