// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 })); // Arc<T>: 複数のスレッドで共有できる値を作成する
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status_locked = status_shared.lock().unwrap(); // Mutexをロックして、共有された値を更新する
            status_locked.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let status_locked = status.lock().unwrap(); // すべてのハンドルが終了した後に、JobStatus.jobs_completedの値を表示する
    println!("jobs completed {}", status_locked.jobs_completed);
}
