use std::sync::mpsc;
use std::thread;

pub fn example() {
    let (tx, rx) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        tx.send(vals).unwrap();
    });

    let thread2 = thread::spawn(move || {
            let received = rx.recv().unwrap();
            println!("{:?}",received);

        for v in received {
            println!("get value : {}",v)
        }


    });

    thread1.join().unwrap();
}