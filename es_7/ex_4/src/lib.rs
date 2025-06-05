use std::{collections::HashMap, thread};
use crossbeam::channel::{unbounded, Sender, Receiver, select};
use rand::Rng;


pub fn start_event_loop() {
    let (cmd_sender, cmd_receiver): (Sender<String>, Receiver<String>) = unbounded();
    let (data_sender, data_receiver): (Sender<(String, i32)>, Receiver<(String, i32)>) = unbounded();
    
    // start data sensors threads
    for i in 0..10 {
        let data_sender = data_sender.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                // Simulate data collection
                let data = (format!("sensor_{}", i), rand::random_range(0..100));
                data_sender.send(data).unwrap();
                thread::sleep(std::time::Duration::from_millis(rand::random_range(10000..20000)));
            }
        });
    }
    
    // simulate command generation
    thread::spawn(move || {
        for _ in 0..10 {
            // Simulate command generation
            let cmd = format!("average sensor_{}", rand::random_range(0..10));
            cmd_sender.send(cmd).unwrap();
            thread::sleep(std::time::Duration::from_millis(rand::random_range(5000..10000)));
        }
    });
    
    
    // start event loop
    let h = thread::spawn(move || {

        let mut data: HashMap<String, Vec<i32>>  = HashMap::new();

        loop {
            select!{
                recv(cmd_receiver) -> cmd => {
                    match cmd {
                        Ok(cmd) => {
                             if cmd.starts_with("average") {
                                 
                                 let sensor = cmd.split_whitespace().nth(1).unwrap();
                                 let values = data.get(sensor);
                                 
                                 if let Some(values) = values {
                                     let avg: f64 = values.iter().map(|&v| v as f64).sum::<f64>() / values.len() as f64;
                                     println!("Average for {}: {}", sensor, avg);
                                 } else {
                                     println!("No data for {}", sensor);
                                 }
                             } else {
                                 println!("Unknown command: {}", cmd);
                             }
                        }
                        Err(_) => {
                            println!("Command channel closed");
                            return;
                        }
                    }
                }
                
                recv(data_receiver) -> datum => {
                    match datum {
                        Ok((sensor, value)) => {
                            println!("Received data from {}: {}", sensor, value);
                            data.entry(sensor).or_insert(Vec::new()).push(value);
                        }
                        Err(_) => {
                            println!("Data channel closed");
                            return;
                        }
                    }
                }
            }
        }
    });

    h.join().unwrap();    
    
}
