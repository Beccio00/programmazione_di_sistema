use clap::{Arg, Parser, Subcommand};
use core::num;
use std::{fs, path::PathBuf};
use lazy_static::lazy_static

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

const bsize: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; bsize]; bsize],
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
       /*  let mut i = 0;
        let b = "B";
        let space = " ";
        let msg = "";

        for boat in boats {
            i = i + 1;
            for k in 1..boat {
                let b_repeted = b.repeat(i) as &str + space;
                msg = msg + b_repeted;            
            }
            fs::write("board.txt", msg);
        }

        */
        let matrix: [[u8; 21]; 20] = [[0; 21]; 20];
        match boats.len() {
            4 => {
                let board = Board {boats, matrix,};
            }
            _ => {
                println!("ERROR");
            }
        }
        

        
    }
    
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String)->Board {}
    
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize))-> Result<Board, Error> {}
    
    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String  {}
}
 

fn run_program() {
    let args = Args::parse();

    match args.command {
        Command::New {values} => {
            let boats: Vec<u8> = values.split(',').map(|s| s.parse().unwrap()).collect();
            let mut i: u8 = 0;
            let mut sum: u32 = 0;

            for boat in &boats {
                i = i + 1;
                sum = sum + (boat*i) as u32;
            }

            match sum {
                1..=17 => {
                    let board = Board::new(&boats);
                }
                _ => {
                    println!("Excessive boats!");
                }
            }
        }

        Command::AddBoat {values} => {
            println!("Command: Add_board, values: {}", values)
        }
    }
}

fn main() {
    run_program();
}
