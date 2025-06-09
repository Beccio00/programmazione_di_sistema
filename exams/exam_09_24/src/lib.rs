/*Gli esercizi di teoria sono stati implementati su vim per evitare di avere aiuti dati dagli errori del compilatore */

//Il seguente programma genera un errore di compilazione. Si descriva il tipo di errore sintattico e si proponga una soluzione
//corretta che permetta ad ogni thread di far avanzare di 100 unità il contatore, con una stampa finale del valore 1000.
//
//    use std::thread;
//    use std::sync::Mutex;
//
//   fn main() {
//       let n = Mutex::new(0);
//       let mut handles = vec![];
//
//       for _ in 0..10 {
//            let handle = thread::spawn(|| {
//                let mut guard = n.lock().unwrap();
//                for _ in 0..100 {
//                    *guard += 1;
//                }
//                println!("New value: {}", guard);
//            });
//            handles.push(handle);
//        }
//
//        for handle in handles {
//            handle.join().unwrap();
//        }
//
//        println!("Final value: {}", n.lock().unwrap());
//    }
//
//
//
// Il problema principale è che Mutex puo avere un solo possessore e quindi è necessario
// incapsularlo all'interno di un Arc. Questo smartpointer (atomic reference counter) che
// corrisponde allo smart pointer RC ma thread safe, mantine il conteggio dei riferimenti possesori
// dell'oggetto che incapsula. È necessario passare allinterno della clousre all'interno di
// thread::spawn il possesso del dato clonato usando la parola chiave move. Di seguito il codice
// con le correzioni spiegate.

pub mod ex_1 {     
   use std::thread;
   use std::sync::{Arc, Mutex};

   pub fn run_ex_1() {
       // let n = Mutex::new(0);

       let shared_data = Arc::new(Mutex::new(0));
       let mut handles = vec![];

       for _ in 0..10 {
            let data  = shared_data.clone();
            let handle = thread::spawn(move || {
                let mut guard = data.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                println!("New value: {}", guard);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("{}", shared_data.clone().lock().unwrap()); 


    }
}

