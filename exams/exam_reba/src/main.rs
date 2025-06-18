use std::{sync::Arc, thread, time::Duration};

use exam_reba::dispatcher::{Dispatcher, Msg};



fn main() {
    let dispatcher = Arc::new(Dispatcher::<String>::new());
    let mut handlers = Vec::new();
    
    for i in 0..3 {  
        let dispatcher_clone = Arc::clone(&dispatcher);
        let handle = thread::spawn(move || {
            let subscription = dispatcher_clone.subscribe();
            
            let mut count = 0;
            while let Some(msg) = subscription.read() {
                println!("Thread {} reads message {}: {:?}", i, count, msg);
                count += 1;
                
                if count >= 3 {
                    break;
                }
            }
        }); 
        handlers.push(handle);
    }

    thread::sleep(Duration::from_millis(100));
    
    dispatcher.dispatch(Msg{value: "Hello".to_string()});   
    dispatcher.dispatch(Msg{value: "World".to_string()});
    dispatcher.dispatch(Msg{value: "!".to_string()});

    for handle in handlers {
        handle.join().unwrap();
    }       
}