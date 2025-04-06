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
        buffer: Vec<Option<T>>,
    }

    impl<T> CircularBuffer<T> {
        pub fn new(cap: usize) -> Self {
            if cap < 0 {
                panic!("{:?}", CircolarBufferError::InvalidCapacity);
            }

            Self { 
                head: 0,
                tail: 0,
                capacity: cap,
                is_empy: true,
                is_full: false,
                buffer: Vec::with_capacity(cap),
            }
        }

        pub fn write(&mut self, item: T) -> Result<&str, CircolarBufferError> {
            if self.is_full {
                Err(CircolarBufferError::BufferFull)
            } else {
                self.buffer[self.tail] = Some(item);
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
                    let item_return = std::mem::take(&mut self.buffer[self.head]);
                    self.head = (self.head + 1) % self.capacity;

                    if self.head == self.tail {
                        self.is_empy = true;
                    }

                    self.is_full = false;
                    item_return
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
            self.buffer[self.head] = Some(item);
        }
        // // vedi sotto*
        pub fn make_contiguous(&mut self) {
            if self.tail < self.head {
                
            }
        }
    }
}
