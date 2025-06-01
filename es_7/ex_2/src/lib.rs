pub mod cyclic_barrier{
    use std::sync::{Arc, Condvar, Mutex};

    struct BarrieState {
        count: usize,
        generation: usize,

    }

    pub struct CyclicBarrier{
        n_thread: usize,
        state: Mutex<BarrieState>,
        cvar: Condvar,
    }

    impl CyclicBarrier{
        pub fn new(n_thread: usize) -> Self {
            CyclicBarrier { 
                n_thread: n_thread, 
                state: Mutex::new(BarrieState{
                    count: 0,
                    generation: 0,
                }),  
                cvar: Condvar::new(),
            }
        }

        pub fn wait(&self) {
            let mut barrierstate = self.state.lock().unwrap();

            let  generation = barrierstate.generation;

            barrierstate.count += 1;

            if barrierstate.count == self.n_thread {
                barrierstate.count = 0;
                barrierstate.generation += 1;
                self.cvar.notify_all();
            } else {
                while generation == barrierstate.generation {
                    barrierstate = self.cvar.wait(barrierstate).unwrap();
                }
            }
        }
    }
}