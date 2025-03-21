use std::{fs, result, sync::WaitTimeoutResult, time::{Duration, SystemTime}};
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    input: String,
}

enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String),

}

pub enum MulErr {
    Overflow, 
    NegativeNumber,
}

use Error::*;

fn print_error(e: Error) {
    match e {
        Simple(time) =>  {
            match time.elapsed() {
                Ok(elepsed) => println!("{:2}s",  elepsed.as_secs()),
                Err(e) => println!("{e:?}"),
            }
        },

        Complex(time, msg ) => {
            match time.elapsed() {
                Ok(elepsed) => println!("{:2}s {}", elepsed.as_secs(), msg),
                Err(e) => println!("{e:?}"),
            }
        },
    }
}


fn run_program(filename: &str) -> io::Result<()> {
    
    match fs::read_to_string(filename) {
        
        Ok(content) => {
            println!("File letto correttamente:\n{}", content);
            let repeated_content = content.repeat(10);
            fs::write(filename, repeated_content)?;
            println!("File aggiornato con il contenuto ripetuto 10 volte!");
        }
        
        Err(e) => {
            eprintln!("Errore nel leggere il file: {}", e);
            return Err(e);
        }
    }

    let prefix = String::from("read_");

    let filename_read = format!("{}{}", prefix, filename);

    match fs::read(filename_read) {
        
        Ok(bytes) => {
            println!("File letto correttamente:\n");

            for byte in bytes {
                print!("{:02x} ", byte);
            }        

            
        }
        Err(e) => {
            eprintln!("Errore nel leggere il file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}


pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> {
    
    if a < 0 || b < 0 {
        return Err(MulErr::NegativeNumber);
    }
    match (a as u32).checked_mul(b as u32) {
        Some(result ) => Ok(result),
        None => Err(MulErr::Overflow),
    }
}

struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {name: name.to_string(), size: 0, count: 0}
    }

    pub fn size(self, n: u32) -> Self{
        Self { name: self.name, size: n, count: self.count }
        
    }

    pub fn count(self, c: u32) -> Self {
        Self { name: self.name, size: self.size, count: c }
    }

    pub fn to_string(&self) {
        println!("name:{} size:{} count:{}", self.name, self.size, self.count);
    }

    pub fn grow(&mut self) {
        self.size = self.size + 1;
    }

    pub fn inc(&mut self) {
        self.count = self.count +1
    } 
}



fn main()  {
    let args = Args::parse();

    run_program(&args.input);

    let time_test = SystemTime::now();

    std::thread::sleep(Duration::from_secs(2));

    let error_0 = Error::Simple(time_test);

    print_error(error_0);

    std::thread::sleep(Duration::from_secs(3));

    let msg = String::from("ERROR: 835510");

    let error_1 = Error::Complex(time_test, msg);

    print_error(error_1);

    let a_0 = -23;
    let b_0 = 58;

    let a_1 = 999999;
    let b_1 = 999999;

    let a_2 = 6;
    let b_2 = 8;

    match mul(a_0, b_0) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            match e {
                MulErr::Overflow => println!("Error: overflow"),
                MulErr::NegativeNumber => println!("Error: negative number"),
            }
        }
    }


    match mul(a_1, b_1) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            match e {
                MulErr::Overflow => println!("Error: overflow"),
                MulErr::NegativeNumber => println!("Error: negative number"),
            }
        }
    }

    match mul(a_2, b_2) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            match e {
                MulErr::Overflow => println!("Error: overflow"),
                MulErr::NegativeNumber => println!("Error: negative number"),
            }
        }
    }

    
    let mut node: Node = Node::new("node").size(10).count(5);

    println!("Node: {} {} {} ", node.name, node.size, node.count);

    node.to_string();

    node.grow();

    node.to_string();

    node.inc();

    node.to_string();
}

