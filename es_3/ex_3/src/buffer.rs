pub mod buffer {

    #[derive(Debug)]
    pub enum CircolarBufferError {
        BufferFull,
        InvalidCapacity,
        BufferEmpy,
    }
    pub struct CircularBuffer<T> {
        head: usize,
        tail: usize,
        capacity: usize,
        is_empy: bool,
        is_full: bool,
        buffer: Vec<T>,
    }

    impl<T: Clone + Default> CircularBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            if capacity < 0 {
                panic!("{:?}", CircolarBufferError::InvalidCapacity);
            }

            let mut buffer = Vec::with_capacity(capacity);

            for _ in 0..capacity {
                buffer.push(T::default());
            }
            Self { 
                head: 0,
                tail: 0,
                capacity,
                is_empy: true,
                is_full: false,
                buffer,
            }
        }

        pub fn write(&mut self, item: T) -> Result<&str, CircolarBufferError> {
            if self.is_full {
                Err(CircolarBufferError::BufferFull)
            } else {
                self.buffer[self.tail] = item;
                self.tail = (self.tail + 1) % self.capacity;

                if self.tail == self.head {
                    self.is_full = true;
                }

                self.is_empy = false;
                Ok("Write in buffer success")
            }
        }

        pub fn read(&mut self) -> Option<T> {
            match self.is_empy {
                false => {
                    let item_return = self.buffer[self.head].clone();
                    self.head = (self.head + 1) % self.capacity;

                    if self.head == self.tail {
                        self.is_empy = true;
                    }

                    self.is_full = false;
                    Some(item_return)
                }

                true => {
                    panic!("{:?}", CircolarBufferError::BufferEmpy);
                }
            }
        }

        pub fn clear(&mut self) {
            self.head = 0;
            self.tail = 0;
            self.is_empy = true;
            self.is_full = false;
        }

        pub fn size(&self) -> usize {
            if self.is_empy {
                0
            } else if self.is_full {
                self.capacity
            } else {
                self.head.abs_diff(self.tail)
            }
        }

        // // può essere usata quando il buffer è pieno per forzare una
        // // scrittura riscrivendo l’elemento più vecchio

        pub fn overwrite(&mut self, item: T) {
            self.buffer[self.head] = item;
        }
        // // vedi sotto*
        pub fn make_contiguous(&mut self) {
            if self.tail < self.head {
                let mut temp_vec = Vec::with_capacity(self.capacity);                
                for i in 0..self.capacity {
                    temp_vec[i] = self.buffer[(self.head + i) % self.capacity].clone();
                }

                self.buffer = temp_vec;                
            }
        }
    }
}
