// Contents
// The experiments module contains a few functions with the solution of the first part of the exercise.
// Clap is used with the builder pattern and subcommands, the derive based solution is commented out.


use std::fs;
use std::usize;

use clap::Parser;
use clap::{arg, command};

pub mod experiments {
    use std::time::SystemTime;

    pub fn rw_file_a(name: &str) {
        match std::fs::read_to_string(name) {
            Ok(data) => {
                if std::fs::write(name, data.repeat(10)).is_ok() {
                    println!("File written");
                } else {
                    println!("Error writing file");
                }
            }
            Err(e) => {
                println!("Error reading file: {}", e);
            }
        }
    }

    pub fn read_file_b(name: &str) {
        let data_s = std::fs::read_to_string(name).unwrap();
        let data = std::fs::read(name).unwrap();
        for c in data_s.chars() {
            print!("{}  ", c);
        }
        println!("");
        for c in data {
            print!("{:02x} ", c);
        }
    }

    enum Error {
        Simple(SystemTime),
        Complex(SystemTime, String),
    }

    fn print_error(e: Error) {
        match e {
            Error::Simple(t) => {
                println!("Simple error: {}", t.elapsed().unwrap().as_nanos());
            }
            Error::Complex(t, s) => {
                println!("Complex error: {} {}", t.elapsed().unwrap().as_nanos(), s);
            }
        }
    }

    pub fn call_print_error() {
        print_error(Error::Simple(SystemTime::now()));
        print_error(Error::Complex(
            SystemTime::now(),
            "Error message".to_string(),
        ));
    }

    pub enum MulError {
        NegativeNumber,
        Overflow,
    }

    pub fn checked_mul(a: i32, b: i32) -> Result<u32, MulError> {
        if a < 0 || b < 0 {
            return Err(MulError::NegativeNumber);
        }

        // we can safely cast to u32 without checking for overflow
        let _a = (if a < 0 { -a } else { a }) as u32;
        let _b = (if b < 0 { -b } else { b }) as u32;

        // here we check for overflow
        if let Some(c) = _a.checked_mul(_b) {
            Ok(c)
        } else {
            Err(MulError::Overflow)
        }
    }

    pub fn call_checked_mul() {
        match checked_mul(0x7fffffff, 0x7fffffff) {
            Ok(c) => {
                println!("Result: {}", c);
            }
            Err(e) => match e {
                MulError::NegativeNumber => {
                    println!("Negative number");
                }
                MulError::Overflow => {
                    println!("Overflow");
                }
            },
        }
    }

    struct Node {
        name: String,
        size: u32,
        count: u32,
    }

    impl Node {
        pub fn new(name: &str) -> Node {
            Node {
                name: name.to_string(),
                size: 0,
                count: 0,
            }
        }

        pub fn size(self, size: u32) -> Self {
            Node { size, ..self }
        }

        pub fn count(self, count: u32) -> Self {
            Node { count, ..self }
        }

        pub fn to_string(&self) -> String {
            format!("{{ {} {} {} }}", self.name, self.size, self.count)
        }

        pub fn grow(&mut self) {
            self.size += 1;
        }
    }

    pub fn call_node() {
        let mut n = Node::new("Node").size(10).count(5);
        println!("Node: {}", n.to_string());
        n.grow();
        println!("Node: {}", n.to_string());
    }
}

const BSIZE: usize = 20;

fn split_nums(s: &str) -> Result<Vec<u8>, ()> {
    // split the boat values and convert to u8
    let mut tokens = Vec::new();
    for token in s.split(",") {
        match token.parse::<u8>() {
            Ok(x) => tokens.push(x),
            Err(_) => return Err(()),
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

#[derive(Debug)]
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    V(usize),
    H(usize),
}

impl Boat {
    // we add a method to parse a string into a Boat (es V12 -> Boat::V(12))
    pub fn from_string(s: &str) -> Result<Boat, String> {
        let c = match s.chars().next() {
            Some(x) => match x {
                'H' | 'V' => x,
                _ => return Err("Boat: invalid boat direction".to_string()),
            },
            None => return Err("Boat: no char found".to_string()),
        };

        let len = match s[1..].parse::<usize>() {
            Ok(x) => x,
            Err(_) => return Err("Boat: can't parse boat length".to_string()),
        };

        match c {
            'H' => Ok(Boat::H(len)),
            'V' => Ok(Boat::V(len)),
            _ => Err("Boat: can't happen, very odd".to_string()),
        }
    }
}

impl Board {
    pub fn new(boats: &[u8]) -> Board {
        let mut b = [0; 4];
        for i in 0..4 {
            b[i] = boats[i];
        }
        Board {
            boats: b,
            data: [[' ' as u8; BSIZE]; BSIZE],
        }
    }

    pub fn from(s: String) -> Board {
        let mut boats: [u8; 4] = [0; 4];
        let mut data: [[u8; BSIZE]; BSIZE] = [[0; BSIZE]; BSIZE];

        let mut i = 0;
        for line in s.lines() {
            if i == 0 {
                let mut j = 0;
                for word in line.split_whitespace() {
                    boats[j] = word.parse().unwrap();
                    j += 1;
                }
            } else {
                let mut j = 0;
                for c in line.chars() {
                    data[i - 1][j] = c as u8;
                    j += 1;
                }
            }
            i += 1;
        }
        Board { boats, data }
    }

    /* true if pos in bounds */
    pub fn in_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 >= 1 && pos.0 <= BSIZE && pos.1 >= 1 && pos.1 <= BSIZE
    }

    /* true if pos overlaps the square an existing boat or it's an immediate neighbour  */
    pub fn cross(&self, pos: (usize, usize)) -> bool {
        // it works because is orgin is (1,1) and not (0,0)
        for i in -1..=1 {
            for j in -1..=1 {
                let x = (pos.0 as isize + i) as usize;
                let y = (pos.1 as isize + j) as usize;

                if !self.in_bounds((x, y)) {
                    continue;
                }

                if self.data[x - 1][y - 1] == 'B' as u8 {
                    return true;
                }
            }
        }

        false
    }

    pub fn add_boat(self, boat: Boat, start: (usize, usize)) -> Result<Board, Error> {
        let mut new_board = self.data;
        let mut boats = self.boats;

        let (len, squares) = match boat {
            Boat::H(len) => {
                let mut squares = vec![];
                for i in 0..len {
                    squares.push((start.0, start.1 + i));
                }
                (len, squares)
            }
            Boat::V(len) => {
                let mut squares = vec![];
                for i in 0..len {
                    squares.push((start.0 + i, start.1));
                }
                (len, squares)
            }
        };

        if self.boats[len - 1] == 0 {
            return Err(Error::BoatCount);
        }

        println!("SQUARES {:?}", squares);

        // add an horizontal boat
        for pos in squares {
            if !self.in_bounds(pos) {
                println!("ssssss {:?}", pos);
                return Err(Error::OutOfBounds);
            }
            if self.cross(pos) {
                return Err(Error::Overlap);
            }
            // it's ok to add
            new_board[pos.0 - 1][pos.1 - 1] = 'B' as u8;
        }

        boats[len - 1] -= 1;
        Ok(Board {
            boats: boats,
            data: new_board,
        })
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in self.boats {
            s.push_str(format!("{} ", i).as_str());
        }
        s.push('\n');

        for i in 0..BSIZE {
            for j in 0..BSIZE {
                s.push(self.data[i][j] as char);
            }
            s.push('\n');
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_empty_board() {
        let b = Board::new(&[4, 3, 2, 1]);
        assert_eq!(b.boats, [4, 3, 2, 1]);
        for i in 0..BSIZE {
            for j in 0..BSIZE {
                assert_eq!(b.data[i][j], ' ' as u8);
            }
        }
    }

    #[test]
    fn add_vertical_boat()  {
        let mut b = Board::new(&[4, 3, 2, 1]);
        b = b.add_boat(Boat::V(3), (1, 1)).unwrap();
        for i in 0..3 {
            assert_eq!(b.data[i][0], 'B' as u8);
        }
        assert!(b.data[3][0] == ' ' as u8);
    }

    #[test]
    fn add_horizontal_boat() {
        let mut b = Board::new(&[4, 3, 2, 1]);
        b = b.add_boat(Boat::H(3), (1, 1)).unwrap();
        for i in 0..3 {
            assert_eq!(b.data[0][i], 'B' as u8);
        }
        assert!(b.data[0][3] == ' ' as u8);
    }

    #[test]
    fn add_boat_out_of_bounds() {
        let mut b = Board::new(&[4, 3, 2, 1]);
        assert!(b.add_boat(Boat::H(3), (1, 20)).is_err());
    }

    #[test]
    fn add_boat_overlap() {
        let mut b = Board::new(&[4, 3, 2, 1]);
        b = b.add_boat(Boat::H(3), (1, 1)).unwrap();
        assert!(b.add_boat(Boat::H(3), (1, 1)).is_err());
    }

}

// example with derive, subcommands can be enums
fn _main() {
    #[derive(Debug, Parser)]
    struct App {
        file: String,

        #[clap(subcommand)]
        subcmd: SubCommand,
    }

    #[derive(Debug, Parser)]
    enum SubCommand {
        Add {
            #[clap(value_delimiter=',')]
            boat: Vec<String>,
        },
        New {
            #[clap(value_delimiter=',', )]
            boat_nums: Vec<String>,
        },
    }
    // we call _main() and handle the error
    let app = App::parse();

    println!("{:?}", app);
}   

// we set a return error to the main, so that we can return a string in case of error
fn main() -> Result<(), String> {
    // uncomment to see the individual experiments in action
    //experiments::rw_file_a("test.txt");
    //experiments::read_file_b("test.txt");
    //experiments::call_print_error();
    //experiments::call_checked_mul();
    //experiments::call_node();

    // naive clap parsing: all arguments are positional
    // with this approach we must collect all the arguments in a single vector and then manually parse them
    //let matches = command!()
    //    .arg(Arg::new("input").action(ArgAction::Append)
    //    ).get_matches();
    // let input = matches.get_many::<String>("input").unwrap().map(|v| v.as_str()).collect::<Vec<_>>();

    // second approach with subcommands and arg! macro (see clap documentation for more details)
    // using subcommand we can individually parse each argument
    // in arg! <> means required argument, [] means optional argument
    // subcommands allow different sets of arguments
    // with the builder pattern we must use get_one() to get the value of an argument 

    let matches = command!()
        .arg(arg!(<file>))
        // add has three parameters
        .subcommand(
            command!("add")
                .arg(arg!(<boat> "boat (Hx or Vx)"))
                .arg(arg!(<start_pos> "start position (row, col), origin (1,1)"))
        )
        // new has two parameters
        .subcommand(
            command!("new")
                .arg(arg!(<boats> "number of size 1, 2, 3, 4 boats, e.g. 6,4,3,2" ))
            )
        .get_matches();
    
    // file is common to all subcommands
    let file = matches.get_one::<String>("file").unwrap();
    
    // handle subcommands add and new
    match matches.subcommand() {
        Some(("add", args)) => {
            let boat_param = args.get_one::<String>("boat").unwrap();
            let start_pos_param = args.get_one::<String>("start_pos").unwrap();

            let boat = match Boat::from_string(boat_param) {
                Ok(x) => x,
                Err(e) => return Err(format!("{:?}", e)),
            };

            let start_pos = match split_nums(&start_pos_param) {
                Ok(x) => (x[0] as usize, x[1] as usize),
                Err(_) => return Err("Invalid start position".to_string()),
            };
            
            if let Ok(data) = fs::read_to_string(file) {
                match Board::from(data).add_boat(boat, start_pos) {
                    Ok(board) => {
                        fs::write(file, board.to_string()).unwrap();
                        println!("Boat {} added at pos {}", boat_param, start_pos_param);
                    }
                    Err(e) => {
                        println!(
                            "Error adding boat {} at pos {}: {:?}",
                            boat_param, start_pos_param, e
                        );
                    }
                }
            } else {
                println!("Error reading file");
            }
        }
        Some(("new", args)) => {
            let boats = args.get_one::<String>("boats").unwrap();

            if let Ok(tokens) = split_nums(&boats) {
                let b = Board::new(&tokens);
                std::fs::write(file, b.to_string()).unwrap();
                println!("New board with boats {} written to {}", boats, file);
            } else {
                return Err("Invalid boat values".to_string());
            }
        }
        _ => {
            println!("No command");
        }
    }

    Ok(())
    
}

