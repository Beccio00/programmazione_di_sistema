pub mod filesystem {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;
    use std::path::Path;
    

    type FSItemCell = RefCell<FSItem>;
    type FSNode = Rc<FSItemCell>;
    type FSNodeWeak = Weak<FSItemCell>;

    struct File {
        name: String,
        parent: FSNodeWeak,
    }
    struct Directory {
        name: String,
        parent: FSNodeWeak,
        children: Vec<FSNode>,
    }
    struct Link {
        name: String,
        target: String,
        parent: FSNodeWeak,
    }
    enum FSItem {
        Directory(Directory),
        File(File), 
        SymLink(Link), 
    }
    
    pub struct FileSystem {
        root: FSNode,
        current_dir: FSNode,
    }

    impl FileSystem {
        // crea un nuovo FS vuoto
        pub fn new()  -> Self {
            let root = Rc::new(RefCell::new(FSItem::Directory(Directory {
                name: String::from("/"),
                parent: Weak::new(),
                children: Vec::new(),
            })));

            let current_dir = root.clone();
            FileSystem { root, current_dir }
        }
        // crea un nuovo FS replicando la struttura su disco
        // pub fn from_disk(path: &Path) -> Self {
        //     let root = FileSystem::build_node(path, None);
        //     let current_dir = root.clone();

        //     FileSystem { root, current_dir }
        // }

        // fn build_node(path: &Path, parent: &Option<FSNodeWeak>) -> FSNode {
        //     let file_name = path.file_name()
        //         .map(|n| n.to_string_lossy().into_owned())
        //         .unwrap_or_else(|| String::from("/"));

        // }

        // cambia la directory corrente, path come in tutti gli altri metodi
        // può essere assoluto o relativo;
        // es: “../sibling” vuol dire torna su di uno e scendi in sibling
        pub fn change_dir(&mut self, path: String) -> Result<(), String>{
            let node = self.find(path);
            if let Some(n) = node {
                self.current_dir = n;
                Ok(())
            } else {
                Err(format!("Directory {} not found", path))
            }
        }

        pub fn find(self, path: String) -> Option<FSNode> {
            unimplemented!()
        }
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