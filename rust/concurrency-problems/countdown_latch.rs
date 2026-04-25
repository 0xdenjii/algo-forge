// Synchronizer - CountDownLatch
// Implement a latch initialised with count N: count_down() decrements it, await()
// blocks until it reaches zero. Used to wait for N workers to finish initialising.
// Approach: Mutex<usize> plus Condvar; notify_all when the count hits zero.
// Time: O(1) per op. Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct CountDownLatch {
    count: Mutex<usize>,
    cv: Condvar,
}

impl CountDownLatch {
    pub fn new(n: usize) -> Self {
        Self { count: Mutex::new(n), cv: Condvar::new() }
    }
    pub fn count_down(&self) {
        let mut c = self.count.lock().unwrap();
        if *c > 0 {
            *c -= 1;
            if *c == 0 {
                self.cv.notify_all();
            }
        }
    }
    pub fn wait(&self) {
        let _g = self.cv.wait_while(self.count.lock().unwrap(), |c| *c > 0).unwrap();
    }
    pub fn wait_timeout(&self, d: Duration) -> bool {
        let (_g, r) = self.cv.wait_timeout_while(self.count.lock().unwrap(), d, |c| *c > 0).unwrap();
        !r.timed_out()
    }
}

fn main() {
    let latch = Arc::new(CountDownLatch::new(5));
    let ready = Arc::new(Mutex::new(Vec::new()));
    for id in 0..5 {
        let (l, r) = (latch.clone(), ready.clone());
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5 * id));
            r.lock().unwrap().push(id);
            l.count_down();
        });
    }
    assert!(!CountDownLatch::new(1).wait_timeout(Duration::from_millis(1)));
    latch.wait();
    assert_eq!(ready.lock().unwrap().len(), 5);
    println!("all services ready: {:?}", ready.lock().unwrap());
}
