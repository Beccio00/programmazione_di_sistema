use std::{thread, sync::Arc, time::Duration};


use ex_7_1::count_down::CountDownLatch;



fn main() {
    demo_latch();
}

pub fn demo_latch() {
    let latch = Arc::new(CountDownLatch::new(1));

    let done_latch = Arc::new(CountDownLatch::new(10));

    let mut handles = vec![];
    for _ in 0..10 {
        let latch_clone = latch.clone();
        let done_latch_clone = done_latch.clone(); 
        let h = thread::spawn(move || {

            latch_clone.wait_zero(None).unwrap();
            doSomeWork("(2) lavoro che necessita driver");
            done_latch_clone.count_down();
            doSomeWork("(3) altro lavoro che non necessita driver");

        });
        handles.push(h);
    }

    doSomeWork("(1) prepapara il driver");
    latch.count_down();

    done_latch.wait_zero(None).unwrap();

    doSomeWork("(4) rilascia il driver");

    for h in handles {
        let _ = h.join();
    }
}

fn doSomeWork(msg: &str) {
    println!("{}", msg);
    std::thread::sleep(Duration::from_millis(10000));
}
