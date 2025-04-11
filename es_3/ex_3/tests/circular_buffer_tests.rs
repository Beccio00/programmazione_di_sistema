use circular_buffer::buffer::{self, buffer::{CircularBuffer, CircularBufferError}};

#[test]
fn test_insert() {
    let mut circular_buffer = CircularBuffer::<i32>::new(10);

    circular_buffer.write(20).unwrap();
    assert_eq!(circular_buffer.size(), 1);

    circular_buffer.write(-2).unwrap();
    assert_eq!(circular_buffer.size(), 2);
}

#[test]
fn test_read() {
    let mut circular_buffer = CircularBuffer::<i32>::new(10);

    circular_buffer.write(5).unwrap();
    assert_eq!(circular_buffer.read(), Some(5));
}

#[test]
fn test_read_n_items() {
    let mut circular_buffer = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        circular_buffer.write(i).unwrap();        
    }

    for i in 0..5 {
        assert_eq!(circular_buffer.read(), Some(i));
    }
} 


#[test]
fn test_return_to_zero() {
    let mut circular_buffer = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        circular_buffer.write(i).unwrap();
    }

    assert_eq!(0, circular_buffer.get_tail());

    for _ in 0..5 {
        circular_buffer.read().unwrap();
    }

    assert_eq!(0, circular_buffer.get_head());
}

#[test]
#[should_panic]
fn test_read_empy_buffer() {
    let mut circular_buffer = CircularBuffer::<i32>::new(5);

    circular_buffer.read();
}


#[test]
fn test_write_full_buffer() {
    let mut circular_buffer = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        circular_buffer.write(i).unwrap();
    }

    assert!(matches!(circular_buffer.write(1), Err(CircularBufferError::BufferFull)));
}

#[test]
fn test_overwrite_buffer() {
    let mut circular_buffer = CircularBuffer::<i32>::new(5);

    for i in 0..4 {
        circular_buffer.write(i).unwrap();
    }

    circular_buffer.overwrite(10);

    assert_eq!(10, circular_buffer.get_buffer()[(5 + circular_buffer.get_tail() - 1) % 5]);

    circular_buffer.overwrite(21);

    assert_eq!(21, circular_buffer.get_buffer()[circular_buffer.get_head()]);
}

#[test]
fn test_make_contiguos() {
    let mut circular_buffer = CircularBuffer::<i32>::new(10);

    for i in 0..10 {
        circular_buffer.write(i).unwrap();
    }

    for _ in 0..7 {
        circular_buffer.read();
    }

    for i in 11..13 {
        circular_buffer.write(i).unwrap();
    }

    circular_buffer.make_contiguous();

    assert_eq!(circular_buffer.get_head(), 0);

    assert_eq!(circular_buffer.get_tail(), 6);
}
