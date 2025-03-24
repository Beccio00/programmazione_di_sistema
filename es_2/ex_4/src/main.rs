use clap::{builder::Str, value_parser, Arg, Parser, Subcommand};
use core::num;
use std::{backtrace::BacktraceStatus, clone, fs};

#[derive(Parser, Debug)] 
struct Args {
    filename: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    New{
        values: String,
    },

    AddBoat{
        values: String,
    },
}

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}

impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        Board { 
            boats: [boats[0], boats[1], boats[2], boats[3]], 
            data: [[' ' as u8; BSIZE]; BSIZE],
        }
    }
    
         
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String)-> Board {
        let mut boats: [u8; 4] = [0; 4];
        let mut data: [[u8; BSIZE]; BSIZE] = [[0; BSIZE]; BSIZE];        
        let mut i = 0;

        for line in s.lines() {
            if i == 0 {
                let mut count_0 = 0;
                let mut count_1 = 0;
                let mut count_2 = 0;
                let mut count_3 = 0;

                for word in line.split_ascii_whitespace() {
                    match word {
                        "B" => count_0 += 1,
                        "BB" => count_1 += 1,
                        "BBB" => count_2 += 1,
                        "BBBB" => count_3 += 1,
                        _=> println!("Error: there are symbols that do not mean nothing")
                    }
                    
                }
                boats = [count_0, count_1, count_2, count_3];
            } else {
                let mut j = 0;
                for char in line.chars() {
                    data[i -1][j] = char as u8;
                    j += 1;
                }
            }
            i += 1;
        }
        Board { boats: boats, data: data }
    }

 
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize))-> Result<Board, Error> {
        let mut d = self.data;
        let mut b = self.boats;



        match boat {
            Boat::Vertical(value) => {
                match b[value - 1] {
                    0 => {
                        return Err(Error::BoatCount);
                        
                    }
                    _ => {
                        for i in 0..value {
                            d[pos.0 + i][pos.1] = 'B' as u8;
                        }
                        b[value - 1] -= 1;
                    }
                }
                
            },
            Boat::Horizontal(value) => {
                match b[value - 1] {
                    0 => {
                        return Err(Error::BoatCount);
                    }
                    _ => {
                        for i in 0..value {
                            d[pos.0][pos.1 + i] = 'B' as u8;
                        }
                        b[value - 1] -= 1;
                    }
                }
                
            },           
        }

        Ok(Board { boats: b, data: d })
    }
    
    /* //converte la board in una stringa salvabile su file */
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
 

fn run_program() -> Result<(), String> {
    let args = Args::parse();
    let filename = &args.filename;

    match args.command {
        Command::New {values} => {
            let boats: Vec<u8> = values.split(',').map(|s| s.parse().unwrap()).collect();
            match boats.len() {
                4 => {
                    let board = Board::new(&boats);
                    fs::write(filename, board.to_string());
                    Ok(())
                    
                }
                _ => {
                    Err(String::from("Error wrong command"))
                } 
            }
        }

        Command::AddBoat {values} => {
            let direction: Vec<u8> = values.split(',').map(|s| s.parse().unwrap()).collect();
            match direction.len() {
                4 => {
                    match fs::read_to_string(filename) {
                        Ok(content) => {
                            let board = Board::from(content);

                            match direction[0] {
                                b'O' => { 
                                    let boat = Boat::Horizontal(direction[1] as usize);
                                    
                                    match board.add_boat(boat, (direction[2] as usize, direction[4] as usize)) {
                                        Ok(new_board) => {
                                            fs::write(filename, new_board.to_string());
                                        },
                                        Err(e) => {
                                            return Err(String::from("Generic error"));
                                        },

                                    }
                                
                                    Ok(())                          
                                }
                                b'V' => { 
                                    let boat = Boat::Vertical(direction[1] as usize);
                                    
                                    match board.add_boat(boat, (direction[2] as usize, direction[4] as usize)) {
                                        Ok(new_board) => {
                                            fs::write(filename, new_board.to_string());
                                        },
                                        Err(e) => {
                                            return Err(String::from("Generic error"));
                                        },

                                    }
                                
                                    Ok(())          
                                }
        
                                _ => {
                                    Err(String::from("Error wrong command"))
                                }
                            }
                        },
                        Err(e) => {
                            Err(String::from(("Error to read the file")))
                        },
                    }
                }
                _ =>{
                    Err(String::from(("Error wrong command")))
                }
            }
        }
    }
/* 
    let string: String = String::from( 
        "B B B B BB BB BB BBB BBB BBBB
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    ");

    let mut board_2 = Board::from(string);
    println!("{:?}\n{:?}", board_2.boats, board_2.data);

    let boat_1 = Boat::Vertical(4);

    match  board_2.add_boat(boat_1, (3,4)) {
        Ok(_) => {
            println!("{:?}\n{:?}", board_2.boats, board_2.data);
        },
        Err(_) => {
            println!("Generic Error");
        },
    }

    let boat_2 = Boat::Horizontal(2);

    match  board_2.add_boat(boat_2, (15,9)) {
        Ok(b) => {
            board_2 = b;
            println!("{:?}\n{:?}", board_2.boats, board_2.data);
        },
        Err(_) => {
            println!("Generic Error");
        },
}
        */
    
    
}

fn main() {
    run_program();
}
