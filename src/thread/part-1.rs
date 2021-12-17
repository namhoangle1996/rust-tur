use std::thread;

pub fn example() {
    // ex1();
    // ex2();
    ex3();

}

fn ex1() {
    thread::spawn(|| {
        for i in 1..19 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}


fn ex2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    // => Thread main sẽ đợi thread được sinh ra thực hiện xong
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // thread::sleep(Duration::from_millis(1));
    }
}


fn ex3() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}