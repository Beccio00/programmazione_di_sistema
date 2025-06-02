pub mod cyclic_barrier {
    use std::sync::{mpsc::{sync_channel, Receiver, SyncSender}, Mutex};

    pub struct Waiter {
        receiver: Receiver<()>,
        senders: Vec<SyncSender<()>>,
    }
    
    pub struct CyclicBarrier{
        waiteras: Mutex<Vec<Waiter>>,
        n_threads: usize,

    }

    impl Waiter {
        fn new() -> Self {
            unimplemented!();
        }

        fn wait() {
            unimplemented!();
        }
    }


    impl CyclicBarrier {
        fn new(n: usize) -> Self {
            unimplemented!();
        }

        fn get_waiter(&self) -> Waiter {
            unimplemented!();
        }
    }


}