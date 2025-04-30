use std::collections::HashMap;

pub mod chrismas_tree {
    struct Tree {
        switches: HashMap<String, bool>,
    
        parents: HashMap<String, String>,
    
        children: HashMap<String, Vec<String>>,
    
        root: String,
    }
    

    impl Tree {
        // nota: aggiustare mutabilità dove necessario gestire errori in caso
        // di collisioni, valori mancanti​        ​

        // aggiungi un nodo figlio del nodo father
        pub fn add(&self, father: &str, node: &str) {}
        // togli un nodo e tutti gli eventuali rami collegati
        pub fn remove(&self, node: &str) {}
    
        // commuta l’interruttore del nodo (che può essere on off) e restituisci il nuovo valore
        pub fn toggle(&self, node: &str) -> bool {}       
        // restituisci se la luce è accesa e spenta
    
        pub fn peek(&self, node: &str) -> bool {}
        }
}