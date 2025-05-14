
// use itertools::{Itertools, Permutations};

// pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
//     if n == 0 {
//         return vec![String::new()];
//     }

//     let mut result = vec![];

//     for &symbol in symbols {
//         for perm in mk_ops(symbols, n - 1) {
//             result.push(format!("{}{}", symbol, perm));
//         }
//     }

//     result
// }

// pub fn prepare(s: &str) -> Vec<String> {

//     let mut result = vec![];
//     let ops = mk_ops(&['+', '-', '*', '/'], 4);
    
//     for digit in s.chars().permutations(s.len()) {
//         for op_seq in &ops {
//             let mut s = String::new();
//             let mut it_op = op_seq.chars();
//             for d in digit.iter() {
//                 s.push(*d);
//                 if let Some(op) = it_op.next() {
//                     s.push(op);
//                 }
//             }
//             result.push(s);
//         }
//     }   
//     result
// }

// #[test]
// fn test_mk_ops() {
//     let symbols = vec!['+', '-', '*', '/'];
//     let n = 4;
//     let result = mk_ops(&symbols, n);
//     assert_eq!(result.len(), symbols.len().pow(n as u32));

//     let res = prepare("23423");
//     println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
// }

// pub fn verify(v: &[String]) -> Vec<String> {}