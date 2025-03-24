use std::path::Display;

trait Summarizable {
    fn summary(&self) -> String {
        String::from("Read more...")
    }    

    fn default_value() -> Self; //vuol dire che deve ritornare un'stantza della rposprio classe

    fn return_none() -> u32 {  // se non c'è il self non ha sensoo, potrebbe essere implementata come funzione normale, solo he in questo caso per invocarla devi richimare l'oggetto
        0
    }
}

impl Summarizable for f64 {
    fn summary(&self) -> String {
        format!("{:.4}", self)
    }

    fn default_value() -> Self {
        0.0
    }
}


impl Summarizable for String {
    fn summary(&self) -> String {
        format!("{:.4}", self)
    }

    fn default_value() -> Self {
        ""
    }
}


impl Summarizable for &str {
    fn summary(&self) -> String {
        if self.len() > 5 {
            format!("{}...{}")
        }
    }
    fn default_value() -> Self {
        ""
    }
}

impl Summarizable for i32 {
    fn summary(&self) -> String {
        
    }

    fn default_value() -> Self {
        0
    }
}

impl Summarizable for i32 {} // lo lascio vuoto

fn print_summary(item: &dyn Summarizable)  {
    println!("{}", item.summary());
    println!("sizeof(item) = {}", std::mem::size_of_val(&item));

}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)] //In questo modo tutto quello che viene scritto dopo viene fatto alla cieca dal compilatore
struct Studente { //es s1=s2 verra prima confrontato il nome, poi il congome poi la matricola se sono tutte uguali allora restituisce true => se ci sono tipi che non sono confrontabili allora da errore => posso agggiungere Debug per avere una modalità stantabile
    nome: String,
    cognome: String,
    matricola: u32,
}


impl Display for Studente {
    fn fmt(&self, f: &mut std::fmt:)
}
/* 

impl PartialOrd for Studente {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.matricola.cmp(&other.matricola))
    }
}

impl Ord for Studente {
    fn partial_cmp(&self,)
}

impl PartialEq for Stuente { //vaolone almeno le proprietà simmetriche e riflessive??
    fn eq(&self, other: &Self) -> bool {
        self.matricola == other.matricola
    }
}

impl Eq for Studente {

}

*/
fn main() {
    let n = 0.1 + 0.2;
    println!("{}", n);
    println!("{}", n.summary());    
    println!("{}", 25.summary());

    let s1 = Studente {
        nome: String::from("Matrio"),
        cognome: String::from("Rossi"),
        matricola: 54321,
    };


    let s2 = Studente {
        nome: String::from("Matrio"),
        cognome: String::from("Rossi"),
        matricola: 12345,
    };

    if s1 == s2 {
        println!(" Gli studenti sono uguali {:?} e {:?}", s1, s2); // lo stampa perchè c'è il dervie Debug
    } else {
        println!("Gli studenti sono diversi")
    }


}
