use std::{collections::VecDeque, fmt::Error, sync::{Arc, Condvar, Mutex}};


pub enum Item<T> {
    Value(T),
    Stop,
}

pub enum ChannelError {
    Closed,

}

struct Channel<T> {
    buffer: Arc<Mutex<VecDeque<Item<T>>>>,
    condvar: Arc<Condvar>,
    capacity: usize,
    closed: Arc<Mutex<bool>>,
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        Channel {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            condvar: Arc::new(Condvar::new()),
            capacity: capacity,
            closed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn write(&self, item: T) -> Result<(), ChannelError> {
        let mut buffer = self.buffer.lock().unwrap();

        loop { // This loop handles the of false wakeup
            let closed = *self.closed.lock().unwrap();            
            if closed {
                return Err(ChannelError::Closed);
            }

            if buffer.len() < self.capacity {
                buffer.push_back(Item::Value(item));
                self.condvar.notify_all();
                return Ok(());
            } 

            buffer = self.condvar.wait(buffer).unwrap();
        }
            
    }

    pub fn read(&self) -> Result<T, ChannelError> {
        let mut buffer = self.buffer.lock().unwrap();

        loop {
            let closed = *self.closed.lock().unwrap();

            if closed && buffer.is_empty(){
                return Err(ChannelError::Closed);
            }

            if let Some(item) = buffer.pop_front() {
                self.condvar.notify_all();
                match item {
                    Item::Value(val) => return Ok(val),
                    Item::Stop => return Err(ChannelError::Closed),                         
                }
            }

            buffer = self.condvar.wait(buffer).unwrap();
    
        }
    }

    pub fn close(&self) {
        let mut closed = *self.closed.lock().unwrap();
        let mut buffer = self.buffer.lock().unwrap();

        closed = true;

        buffer.push_back(Item::Stop);

        self.condvar.notify_all();
    }


}