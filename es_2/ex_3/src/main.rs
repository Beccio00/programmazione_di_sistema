use std::fs;
use std::error::Error;
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    input: String,
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

    Ok(())
}




fn main()  {
    let args = Args::parse();

    run_program(&args.input);
}
