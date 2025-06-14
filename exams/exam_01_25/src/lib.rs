// La struct DelayedExecutor permette di eseguire funzioni in modo asincrono, dopo un certo intervallo di tempo.
// Essa offre tre metodi:
/// ```
/// new() -> Self //crea un nuovo DelayedExecutor

/// execute<F: FnOnce()+Send+'static>(f:F, delay: Duration) -> bool
// se il DelayedExecutor è aperto, accoda la funzione f che dovrà essere eseguita non prima che sia
// trascorso un intervallo pari a delay e restituisce true;
// se invece il DelayedExecutor è chiuso, restituisce false.

/// close(drop_pending_tasks: bool) chiude il DelayedExecutor;
// se drop_pending_tasks è true, le funzioni in attesa di essere eseguite vengono eliminate, altrimenti
// vengono eseguite a tempo debito.
// DelayedExecutor è thread-safe e può essere utilizzato da più thread contemporaneamente.
// I task sottomessi al DelayedExecutor devono essere eseguiti in ordine di scadenza.
// All'atto della distruzione di un DelayedExecutor, tutti i task in attesa sono eliminati, ma se è in corso
// un'esecuzione questa viene portata a termine evitando di creare corse critiche.Si implementi questa struct in linguaggio Rust.

mod ex_4 {
    use std::{collections::BinaryHeap, sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::Instant};


    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub enum DelayedExecutorstate {
        Open,
        Close,
    }

    pub struct Task {
        job: Job,
        instant: Instant,
    } 

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.instant.cmp(&self.instant)
        }
    }

    impl PartialEq for Task {
        fn eq(&self, other: &Self) -> bool {
            self.instant == other.instant
        }

        fn ne(&self, other: &Self) -> bool {
            self.instant != other.instant
        }
    }

    impl Eq for Task {}

    pub struct DelayedExecutor {
        shared: Arc<Mutex<(BinaryHeap<Task>, DelayedExecutorstate)>>,
        cv: Arc<Condvar>,
        jh: JoinHandle<()>,
    }

    impl DelayedExecutor {
        pub fn new() -> Self {
            let shared = Arc::new(Mutex::new((BinaryHeap::new(), DelayedExecutorstate::Open)));
            let convar = Arc::new(Condvar::new());

            let share_clone = Arc::clone(&shared);
            let convar_clone = Arc::clone(&convar);


            let handler = thread::spawn(move || {
                loop {
                    let now = Instant::now();

                    let (heap, state) = &*share_clone.lock().unwrap();

                    match *state {
                        DelayedExecutorstate::Close => {break;}
                        DelayedExecutorstate::Open => {
                            if let Some(task) = heap.peek() {
                                if (*task).instant <= now {


                                }
                            }
                        }
                    }
                    
                    
                     


                }
            });

            DelayedExecutor { shared: shared, cv: convar, jh: handler }
        }
    }

    

}