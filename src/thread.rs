use std::thread;
use std::time::Duration;

pub fn run() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // thread::sleep(Duration::from_millis(2));
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
