use channel::{Channel, ChannelError};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let buffer_size = 5;
    let num_items = 100;
    let channel = Channel::<i32>::new(buffer_size);

    let consumer_channel = channel.clone();

    let producer_thread = thread::spawn(move || {
        let mut rng = rand::rng();

        for i in 0..num_items {
            match channel.write(i) {
                Ok(_) => println!("Produced: {}", i),
                Err(e) => {
                    println!("Producer: Channel closed: {:?}", e);
                    break;
                }
            }
            let delay = rng.random_range(50..500);
            thread::sleep(Duration::from_millis(delay));
        }

        println!("Producer: Finished producing items");
        channel.close();
        println!("Producer: Channel closed");
    });

    let consumer_thread = thread::spawn(move || {
        let mut rng = rand::rng();
        loop {
            match consumer_channel.read() {
                Ok(item) => println!("Consumed: {}", item),
                Err(ChannelError::Closed) => {
                    println!("Consumer: Channel closed");
                    break;
                }
            }
            let delay = rng.random_range(50..500);
            thread::sleep(Duration::from_millis(delay));
        }
        println!("Consumer: Finished consuming items");
    });

    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    println!("Main thread: All done!");
}
