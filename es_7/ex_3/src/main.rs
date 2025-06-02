
fn main() {
    let cbarrrier = CyclicBarrier::new(3);
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
}
