use std::{collections::VecDeque, fmt::Error, sync::{Arc, Condvar, Mutex}};


pub enum Item<T> {
    Value(T),
    Stop,
}

struct Channel<T> {
    buffer: Arc<Mutex<VecDeque<Item<T>>>>,
    condvar: Arc<Condvar>,
    capacity: usize,
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        Channel {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            condvar: Arc::new(Condvar::new()),
            capacity,
        }
    }

    pub fn write(&self, item: T) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn read() -> Result<(), Error> {
        unimplemented!();
    }

    pub fn close() {
        unimplemented!();
    }


}