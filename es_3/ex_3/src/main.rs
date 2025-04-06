use crate::buffer::buffer::{CircularBuffer, CircolarBufferError};


mod buffer;


fn main() {
  let buffer = CircularBuffer<u32>::new(10);

}
