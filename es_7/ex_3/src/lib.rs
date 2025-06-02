pub mod cyclic_barrier {
    use std::sync::{mpsc::{channel, sync_channel, Receiver, Sender}, Mutex};

    pub struct Waiter {
        receiver: Receiver<()>,
        senders: Vec<Sender<()>>,
    }

    impl Waiter {
        pub fn wait(&self) {
            self.senders.iter().for_each(|s| {
                s.send(()).unwrap();
            });
    
            for _ in 0..self.senders.len() {
                self.receiver.recv().unwrap();
            }
        }
    }
    
    
    pub struct CyclicBarrier{
        receivers: Vec<Receiver<()>>,
        seners: Vec<Sender<()>>
    }

    impl CyclicBarrier {
        pub fn new(n: usize) -> Self {
            let mut senders = Vec::new();
            let mut recivers = Vec::new();

            for _ in 0..n {
                let (tx, rx) = channel();

                senders.push(tx);
                recivers.push(rx);    

            }

            CyclicBarrier { receivers: recivers, seners: senders }
        }

        pub fn get_waiter(&mut self) -> Result<Waiter, ()> {
            match self.receivers.len() {
                0 => {
                    Err(())
                },
                _ => {
                    let idx = self.seners.len() - self.receivers.len();
                    let receiver = self.receivers.pop().unwrap();
                    let senders = self.seners.iter().enumerate()
                        .filter(|(i,_)| *i != idx )
                        .map(|(_,s)| s.clone()).collect();
                    
                    Ok(Waiter { receiver: receiver, senders: senders })
                }
            }
        }
    }

}