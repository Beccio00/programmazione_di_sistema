// we divived the business logic in functions in order to make tests easier
// each performs a single task

fn stats(text: &str) -> [u32; 26] {
    let mut counts = [0; 26];

    // we must iterate over chars, not bytes   
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // this works as in C for char - 'a'
            // just remember that in Rust chars are Unicode, so we must convert to ASCII
            // check the real size of a char in Rust, it's not 1 byte, but for ascii values
            // the code is teh same
            let i = c.to_ascii_lowercase() as usize - 'a' as usize;
            counts[i] += 1;
        }
    }
    counts
}

fn is_pangram(counts: &[u32]) -> bool {
    
    if counts.len() != 26 {
        return false;
    }

    // with iterators (we'll see later in the course)
    // is_pangram = counts.iter().all(|&c| c > 0)
    
    for c in counts {
        if *c == 0 {
            return false;
        }
    }

    // as a different option you can use "pattern matching" to extract the value 
    // from the reference in the loop
    // in "for xx in iterable" xx is a reference to the value in the interable.
    // using &x you can extract the value from the reference since the types
    // must match 

    // for &c in counts {
    //     if c == 0 {
    //         return false;
    //     }
    // }

    true
}

// call this function from main
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    
    let contents = std::fs::read_to_string(&args[1]).expect("cannot read file");

    let counts = stats(&contents);

    if is_pangram(&counts) {
        println!("\"{}\" is a pangram!", contents);
    } else {
        println!("Not a pangram!");
    }

    for (i, c) in counts.iter().enumerate() {
        println!("{}: {}", (i as u8 + 'a' as u8) as char, c);
    };
}


// please note, code has been splittend in simple functions in order to make tests easier
#[cfg(test)] // this is a test module
mod tests
{   
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

