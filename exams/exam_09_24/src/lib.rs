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
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn run_ex_1() {
        // let n = Mutex::new(0);

        let shared_data = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let data = shared_data.clone();
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

// fn main() {
//     let mut valore = Rc::new(5);
//     {
//         println!("Value: {:?}", valore);

//         let copia = Rc::clone(&valore);
//         println!("Copied value: {:?}", copia);

//         match Rc::get_mut(&mut valore) {
//             Some(v) => *v += 10,
//             None => println!("It seems that something had been wrong (case A)"),
//         }
//     }
//     match Rc::get_mut(&mut valore) {
//         Some(v) => *v += 10,
//         None => println!("It seems that something had been wrong (case B)"),
//     }
//     println!("The final value is: {:?}", valore);
// }

//Si indichino gli eventuali errori di compilazione, oppure, in assenza di errori di compilazione si descriva
// il comportamento del programma, indicando le stringhe visualizzate in output e giustificando la risposta.

// non ci sono errori di compilazione. Il comportatmento del programma è il seguente: in valore è uno smart pointer
// che punta a una variabile di valore 5, successivamente viene fatta un Rc::clone di valore in copia, che è una
// shallow copy del puntatore che incrementa il counter di Rc. Dopo si prova a modifacere il contenuto di valore
// ma dato che valore non è l'unico puntatore e counter = 2 Rc::get_mut(&mut valore) sarà equivalente a None e
// verrà stampato a schermo "It seems that something had been wrong (case A)". Dopodichè la copia esce di scope
// il counter dell'Rc ritorna a 1 ed è quindi possibile modificare il contenuto di valore e l'ultimo print stamperà
// "The final value is: 15"
pub mod ex_2 {
    use std::rc::Rc;

    pub fn run_ex_2() {
        let a = 5;
        let b = &a;
        let mut valore = Rc::new(a);

        println!("value: {}", b);
        {
            println!("Value: {:?}", valore);

            let mut copia = Rc::clone(&valore);

            println!("Copied value: {:?}", copia);

            match Rc::get_mut(&mut valore) {
                Some(v) => *v += 10,
                None => println!("It seems that something had been wrong (case A)"),
            }

            println!("Value: {:?}", valore);

            match Rc::get_mut(&mut copia) {
                Some(v) => *v += 20,
                None => println!("It seems that something had been wrong (case A)"),
            }
            println!("Value: {:?}", valore);
            println!("Copied value: {:?}", copia);
        }
        match Rc::get_mut(&mut valore) {
            Some(v) => *v += 10,
            None => println!("It seems that something had been wrong (case B)"),
        }
        println!("The final value is: {:?}", valore);
    }
}
// Dati i seguenti programma, per ciascuno di essi, si motivi l'errore sintattico rilevato dal compilatore e si effettui la correzione
// necessaria per correggerli, avendo (per entrambi) come obiettivo la generazione di una stringa risultato sulla base del
// confronto delle lunghezze delle stringhe string1 e string2.


// fn fun1<'a>(x: &'a str, y: &a' str) -> &'a str {
//     if x.len() < y.len() {
//         x
//     } else {
//         &x[0..y.len()] 
//     }
// }

// fn main() {
//     let string1 = String::from("torino");
//     let result;
//     {
//         let string2 = String::from("2024");
//         result = fun1(string1.as_str(), string2.as_str());
//     }
//     println!("The fun string is {}", result);
// }

// fn fun2<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() < y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("ciao mamma");
//     let result;
//     {
//         let string2 = String::from("Torino");
//         result = fun2(string1.as_str(), string2.as_str());
//     }
//     println!("The fun string is {}", result);
// }


// In questo caso nel 
pub mod ex_3 {
    pub fn fun1<'a>(x: &'a str, y: & str) -> &'a str {
        if x.len() < y.len() { 
            x 
        } else { 
            &x[0..y.len()] 
        }
    }

    pub fn fun2<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() < y.len() {
            x 
        } else {
            y 
        }
    }
}


/*
Un CancelableLatch è un tratto di sincronizzazione che permette
a uno o più thread di attendere, senza consumare cicli di CPU, che
altri thread eseguano i propri compiti e ne segnalino l’esito.
All’atto della creazione di questa struttura viene indicato il numero
di compiti da attendere. La struttura offre il metodo count_down()
che permette di indicare che uno dei compiti è terminato con successo:
se non restano altri compiti da attendere, le attese vengono sbloccate
con successo, altrimenti proseguono.
Il metodo cancel() permette di segnalare che uno dei compiti è fallito:
in questo caso, le attese vengono subito sbloccate indicando l’avvenuta
cancellazione.
La struttura offre due metodi di attesa: uno incondizionato (ovvero,
l’attesa si protrae fino a che tutti i compiti sono stati terminati
con successo o è stata richiesta una cancellazione) e uno con timeout
(in questo caso, l’attesa può terminare anche se entro il tempo
indicato non si raggiungono le condizioni precedenti: in tale caso
viene segnalato che il tempo è scaduto).

Si realizzi, usando il linguaggio Rust, una struttura che implementi tale tratto
*/


pub mod ex_4 {
    use std::{io::repeat, sync::{Condvar, Mutex}, time::Duration};
        
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum WaitResult {
        Success,
        Timeout,
        Canceled
    }

    pub struct LatchState {
        counter: usize,
        cancelled: bool,
    }

    pub struct Latch {
        state: Mutex<LatchState>,
        cv: Condvar,
    }

    pub trait CancelableLatch {
        fn new(count: usize) -> Self;
        fn count_down(&self);
        fn cancel(&self);
        fn wait(&self) -> WaitResult;
        fn wait_timeout(&self, d: Duration) -> WaitResult;
    }

    impl CancelableLatch for Latch {
        fn new(count: usize) -> Self {         

            Latch { 
                state: Mutex::new(LatchState { counter: count, cancelled: false }), 
                cv: Condvar::new(),
            }
        }

        fn count_down(&self) { 
            let mut guard = self.state.lock().unwrap();

            if guard.counter > 0 {
                guard.counter -= 1;
                if guard.counter == 0 {
                    self.cv.notify_all();
                }
            }
            
        }
        
        fn cancel(&self) { 
            let mut guard = self.state.lock().unwrap();

            guard.cancelled = true;

            self.cv.notify_all();

        }
        
        fn wait(&self) -> WaitResult{
            let guard = self.state.lock().unwrap();

            let data = self.cv.wait_while(guard, |s| s.counter > 0 && !s.cancelled).unwrap();
                
            if data.cancelled {
                WaitResult::Canceled
            } else  {
                WaitResult::Success
            } 
        }
        
        fn wait_timeout(&self, d: Duration) -> WaitResult { 
            let guard = self.state.lock().unwrap();

            let data = self.cv.wait_timeout_while(guard, d, |s| s.counter > 0 && !s.cancelled).unwrap();            

            if data.0.cancelled {
                WaitResult::Canceled
            } else if data.1.timed_out() {
                WaitResult::Timeout
            } else {
                WaitResult::Success
            }
            
        }
    }

}

mod test {
    use crate::ex_4::{CancelableLatch, Latch, WaitResult};
    use std::thread;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_multi_thread_cancel() {
        let latch = Arc::new(Latch::new(100)); 
        let mut handles = Vec::new();
    
        for _ in 0..5 {
            let latch_clone = Arc::clone(&latch);
            let handle = thread::spawn(move || {
                latch_clone.count_down();
            });
            handles.push(handle);
        }
    
        let latch_canceller = Arc::clone(&latch);
        let canceller = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            latch_canceller.cancel();
        });
    
        let latch_waiter = Arc::clone(&latch);
        let waiter = thread::spawn(move || {
            latch_waiter.wait()
        });
    
        for handle in handles {
            handle.join().unwrap();
        }
        canceller.join().unwrap();
    
        let result = waiter.join().unwrap();
        assert_eq!(result, WaitResult::Canceled);
    }
}