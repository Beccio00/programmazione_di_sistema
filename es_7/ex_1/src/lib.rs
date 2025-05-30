pub mod count_down{
    use std::{result, sync::{Arc, Condvar, Mutex}, time::Instant};

    #[derive(Clone)]
    pub struct CountDownLatch {
        shared: Arc<(Mutex<usize>, Condvar)>,
    }

    impl CountDownLatch {
        pub fn new(n: usize) -> Self {
            CountDownLatch { 
                shared: Arc::new((Mutex::new(n), Condvar::new())),                
            }
        }

        // wait zero aspetta al massimo timeout ms
        // se esce per timeout ritorna Err altrimenti Ok
        pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),()> {
            let (mutex, cv) = &*self.shared;

            let mut counter = mutex.lock().unwrap();

            if *counter == 0 {
                return Ok(());
            }

            if let Some(duration) = timeout {
                let now = Instant::now();

                while *counter > 0 {
                    let result = cv.wait_timeout(counter, duration).unwrap();
                    counter = result.0;

                    if result.1.timed_out() { //timed i
                        return Err(());
                    }
                }
            } else {
                while *counter > 0  {
                    counter = cv.wait(counter).unwrap();
                }
            }
            Ok(())

        }

        pub fn count_down(&self) {
            let (mutex, cv) = &*self.shared;
            
            let mut counter = mutex.lock().unwrap();

            if *counter > 0 {
                *counter -= 1;
                if *counter == 0 {
                    cv.notify_all();
                }                
            } 
        }
    }
}
