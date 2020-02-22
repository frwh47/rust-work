use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    f1();
    f2();
    f3();
    f4();
}

fn f1() {
    let handler = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(10));
    }

    handler.join().unwrap();
}

fn f2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    //drop(v);
    handle.join().unwrap();
}

fn f3() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(2));
    });
    //handle.join().unwrap();
    println!("{:?} got {:?}", rx, rx.recv().unwrap());
}

fn f4() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["h1", "from", "the", "thread"];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got: {}", received);
    }
}
