use std::sync::{Arc, Mutex};
use std::thread;

pub fn example() {
    // let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut counter = 1;

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("run thread {:?}",i);
            counter += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter);
}



