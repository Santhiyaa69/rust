use std::thread;
use std::time::Duration;

pub fn run() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
