use std::thread;
use std::time::Duration;

fn main() {
    let thread_1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });

    let thread_2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(4));
        }
    });

    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(8));
    }

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}
