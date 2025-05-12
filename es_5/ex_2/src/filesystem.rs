pub mod filesystem {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    struct File {
        name: String,
        parent: Weak<RefCell<FSItem>>,
    }
    #[derive(Clone)]
    struct Directory {
        name: String,
        parent: Weak<RefCell<FSItem>>,
        children: Vec<Rc<RefCell<FSItem>>>,
    }
    struct Link {
        name: String,
        path: String,
        target: String,
    }
    enum FSItem {
        Directory(Directory),
        File(File), 
        SymLink(Link), 
    }
    pub struct FileSystem {
        root: Directory,
        current_dir: Directory,
    }

    impl FileSystem {
        // crea un nuovo FS vuoto
        pub fn new()  -> Self {
            let root = Directory {
                name: String::from("/"),
                parent: Weak::new(),
                children: Vec::new(),
            };
            let current_dir = root.clone();
            FileSystem { root, current_dir }
        }
        // // crea un nuovo FS replicando la struttura su disco
        // pub fn from_disk() -> Self {
        //     unimplemented!();
        // }
        // // cambia la directory corrente, path come in tutti gli altri metodi
        // // può essere assoluto o relativo;
        // // es: “../sibling” vuol dire torna su di uno e scendi in sibling
        // pub fn change_dir(&mut self, path: String) -> Result {
        //     unimplemented!();
        // }
        // // crea la dir in memoria e su disco
        // pub fn make_dir(&self, path: String, name: String) -> Result {
        //     unimplemented!();
        // }
        
        // // crea un file vuoto in memoria e su disco
        // pub fn make_dir(&self, path: String, name: String) -> Result {
        //     unimplemented!();
        // }
        // // rinonima file / dir in memoria e su disco
        // pub fn rename(&self, path: String, new_name: String) -> Result {
        //     unimplemented!();
        // }
        // // cancella file / dir in memoria e su disco, se è una dir cancella tutto il contenuto
        // pub fn delete(&self, path: String) -> Result {
        //     unimplemented!();
        // }
        // // cerca l’elemento indicato dal path e restituisci un riferimento
    }

}