// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));

            let mut status_shared = status_shared.lock().unwrap();
            // TODO: You must take an action before you update a shared value.
            status_shared.jobs_done += 1;
            start.elapsed()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    // Waiting for all jobs to complete.
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    for (idx, handle) in results.into_iter().enumerate() {
        println!("Thread {} took {:?}", idx, handle);
    }
    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
