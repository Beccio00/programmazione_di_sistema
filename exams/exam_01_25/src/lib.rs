// La struct DelayedExecutor permette di eseguire funzioni in modo asincrono, dopo un certo intervallo di tempo.
// Essa offre tre metodi:
/// ```
/// new() -> Self //crea un nuovo DelayedExecutor

/// execute<F: FnOnce()+Send+'static>(f:F, delay: Duration) -> bool
// se il DelayedExecutor è aperto, accoda la funzione f che dovrà essere eseguita non prima che sia
// trascorso un intervallo pari a delay e restituisce true;
// se invece il DelayedExecutor è chiuso, restituisce false.

/// close(drop_pending_tasks: bool) //chiude il DelayedExecutor;
// se drop_pending_tasks è true, le funzioni in attesa di essere eseguite vengono eliminate, altrimenti
// vengono eseguite a tempo debito.
// DelayedExecutor è thread-safe e può essere utilizzato da più thread contemporaneamente.
// I task sottomessi al DelayedExecutor devono essere eseguiti in ordine di scadenza.
// All'atto della distruzione di un DelayedExecutor, tutti i task in attesa sono eliminati, ma se è in corso
// un'esecuzione questa viene portata a termine evitando di creare corse critiche.Si implementi questa struct in linguaggio Rust.

mod ex_4 {
    use std::{collections::{self, BinaryHeap}, sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant}};


    type Job = Box<dyn FnOnce() + Send + 'static>;

    #[derive(PartialEq)]
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
        jh: Option<JoinHandle<()>>,
    }

    impl DelayedExecutor {
        pub fn new() -> Self {
            let shared = Arc::new(Mutex::new((BinaryHeap::<Task>::new(), DelayedExecutorstate::Open)));
            let convar = Arc::new(Condvar::new());

            let share_clone = Arc::clone(&shared);
            let convar_clone = Arc::clone(&convar);


            let handler = thread::spawn(move || {
                loop {
                    let now = Instant::now();

                    let mut guard = share_clone.lock().unwrap();

                    guard = convar_clone.wait_while(guard, |g| {
                        match g.0.peek() {
                            Some(task) => task.instant > Instant::now() && g.1 == DelayedExecutorstate::Open,
                            None => g.1 == DelayedExecutorstate::Open
                        }
                    }).unwrap();

                    if guard.1 == DelayedExecutorstate::Close {
                        break;
                    }

                    if let Some(task) = guard.0.peek() {
                        if task.instant <= Instant::now() {
                            let task = guard.0.pop().unwrap();
                            drop(guard);
                            (task.job)();
                        }
                    }                    

                }
            });

            DelayedExecutor { shared: shared, cv: convar, jh: Some(handler) }
        }

        pub fn execute<F: FnOnce()+Send+'static>(&self, f:F, delay: Duration) -> bool {
            
            
            let  (heap, state) = &mut *self.shared.lock().unwrap();
            match *state {
                DelayedExecutorstate::Close => {
                    false
                },

                DelayedExecutorstate::Open => {
                    heap.push(Task { job: Box::new(f), instant: Instant::now() + delay });

                    self.cv.notify_all();
                    true                    
                }
            }


        }

        pub fn close(&self, drop_pending_task: bool) {
            let (heap, state) = &mut *self.shared.lock().unwrap();
            
            if drop_pending_task {
                heap.clear();
            }
            
            *state = DelayedExecutorstate::Close;
            self.cv.notify_all();
        }
    }

    impl Drop for DelayedExecutor {
        fn drop(&mut self) {
            self.close(true);
            self.jh.take().unwrap().join().unwrap();
        }
    }
    

}


#[cfg(test)]
mod tests {
    use super::ex_4::{DelayedExecutor, DelayedExecutorstate};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_delayed_executor() {
        let executor = DelayedExecutor::new();
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        executor.execute(move || {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        }, Duration::from_millis(100));

        thread::sleep(Duration::from_millis(200));
        assert_eq!(*counter.lock().unwrap(), 1);
        
        executor.close(true);
    }
}