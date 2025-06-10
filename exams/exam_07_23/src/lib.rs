mod delayed_queue {
    use std::{cmp::Ordering, collections::BinaryHeap, sync::{Condvar, Mutex}, time::Instant};


    struct Item<T:Send> {
        t: T,
        i: Instant,
    }

    impl<T: Send> PartialEq for Item<T> {
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }


    impl<T: Send> Eq for Item<T> {}

    impl<T:Send> PartialOrd for Item<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.i.partial_cmp(&self.i)
        } 
    }

    impl<T: Send> Ord for Item<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&self.i)
        }
    }


    pub struct DelayedQueue<T: Send> {
        data: Mutex<BinaryHeap<Item<T>>>, //ho creato un binary key con un tupla ma poi è difficle mantenerala ordinata
        cv: Condvar

    }

    impl<T:Send> DelayedQueue<T> {
        pub fn new() -> Self {
            DelayedQueue {
                data: Mutex::new(BinaryHeap::new()),
                cv: Condvar::new(),
            }
        }

        pub fn offer(&self, t: T, i: Instant) {
            let mut data = self.data.lock().expect("Failed to lock mutex");
            data.push(Item { t, i });
            drop(data);
            self.cv.notify_all() 

        }

        pub fn take(&self) -> Option<T>  {
            let mut data = self.data.lock().expect("Mutex poisoned");
            loop {
                let now = Instant::now();
                let first = data.peek();

                if let Some(item) = first { //se provando data.peek() esce Some bella altrimeni None va nell'else
                    let i = item.i;
                    if i < now  {
                        let res = data.pop().unwrap();
                        return Some(res.t);
                    } else { //devo dormire fino per un ora, perchè ritengo che fra un'ora le cose sono a posto
                        data = self.cv.wait_timeout(data, i.duration_since(now)).expect("Mutex.poisoned").0; //per wait_timeout restituisce una tupla io devo prendere il primo elemento
                    }
                }else {
                    return None;
                }
            }
        }

        pub fn size(&self) -> usize {
            let data = self.data.lock().expect("Mutex poisoned");
            data.len()
        }
    }

}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::delayed_queue::DelayedQueue;

    #[test]
    fn an_empty_queue_returns_none() {
        let q = DelayedQueue::<i32>::new();
        let now = Instant::now();
        q.offer(1500, now + Duration::from_millis(10));
        q.offer(1500, now + Duration::from_millis(5));
        assert_eq!(q.take(), Some(1500));
        assert_eq!(q.take(), Some(1500));
        assert_eq!(q.take(), None);

    }

    // da rifare
    #[test]
    fn items_are_returned_in_order_even_if_insered_after_waitingstarts() -> () {
        let q = DelayedQueue::new();
        std::thread::scope(|s| {
            let now = Instant::now();
            q.offer(43, now + Duration::from_millis(10));
            let handle1 = s.spawn( || {
                assert_eq!(q.take(), Some(20));
            });
            // FIXME: THERE IS AN ERROR HERE
            
            // let handle2 = s.spawn(|| {
            //     std::thread::sleep(Duration::from_millis(2));
            //     q.offer(20, now + Duration::from_millis(1));
            // });
            handle1.join().expect("Thread 1 panicked");
            // handle2.join().expect("Thread 2 panicked");
        });
    }


    #[test]
    fn mothod_size_work() {
        let q = DelayedQueue::new();
        q.offer(1500, Instant::now());
        assert_eq!(q.size(), 1);
    }
    #[test]
    fn two_threads_reading_the_queue_work() {
        let q = DelayedQueue::new();
        q.offer(1500, Instant::now() + Duration::from_millis(10));
        q.offer(500, Instant::now() + Duration::from_millis(5));
        std::thread::scope(|s| {
            for _ in 0..2 {
                s.spawn(|| {
                    let r = q.take();
                    assert!(r == Some(1500) || r == Some(500));
                });
            }
        });
        
    }

}