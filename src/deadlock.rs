use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    let thread1 = thread::spawn(move || {
        let _guard1 = mutex1_clone.lock().unwrap();
        println!("Thread 1 acquired mutex1");

        // Introducing a delay to increase the chances of deadlock
        thread::sleep(std::time::Duration::from_secs(1));

        let _guard2 = mutex2_clone.lock().unwrap();
        println!("Thread 1 acquired mutex2");
    });

    let thread2 = thread::spawn(move || {
        let _guard2 = mutex2.lock().unwrap();
        println!("Thread 2 acquired mutex2");

        let _guard1 = mutex1.lock().unwrap();
        println!("Thread 2 acquired mutex1");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}