use std::thread;
use std::sync::{Arc, Mutex};

fn increment_counter(counter: Arc<Mutex<i32>>) {
    // This function increments the counter within a mutex lock
    let mut count = counter.lock().unwrap();
    *count += 1;
}

fn counter() -> i32 {
    // Create a mutable counter shared among threads
    let counter = Arc::new(Mutex::new(0));

    // Create 10 threads and trigger the increment_counter function in each thread
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            increment_counter(counter);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Get the final count from the shared counter
    let final_count = *counter.lock().unwrap();
    final_count
}

fn main() {
    let mut value = 0;
    for _ in 0..10 {
        value = value + counter();
    }
    println!("{}", value)
}
