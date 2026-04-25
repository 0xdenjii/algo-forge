// Synchronizer - Counting Semaphore (connection-pool limiter)
// Implement a semaphore with acquire/release that limits how many threads may use a
// resource at once; verify that concurrency never exceeds the permit count.
// Approach: Mutex<usize> permits + Condvar; acquire returns an RAII permit that
// releases on drop, so permits cannot leak on early return or panic.
// Time: O(1) per op. Space: O(1).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct Semaphore {
    permits: Mutex<usize>,
    cv: Condvar,
}

pub struct Permit<'a>(&'a Semaphore);

impl Semaphore {
    pub fn new(n: usize) -> Self {
        Self { permits: Mutex::new(n), cv: Condvar::new() }
    }
    pub fn acquire(&self) -> Permit<'_> {
        let mut p = self.cv.wait_while(self.permits.lock().unwrap(), |p| *p == 0).unwrap();
        *p -= 1;
        Permit(self)
    }
    pub fn try_acquire(&self) -> Option<Permit<'_>> {
        let mut p = self.permits.lock().unwrap();
        if *p == 0 { return None; }
        *p -= 1;
        Some(Permit(self))
    }
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        *self.0.permits.lock().unwrap() += 1;
        self.0.cv.notify_one();
    }
}

fn main() {
    let sem = Arc::new(Semaphore::new(3));
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let hs: Vec<_> = (0..12)
        .map(|_| {
            let (sem, active, peak) = (sem.clone(), active.clone(), peak.clone());
            thread::spawn(move || {
                let _p = sem.acquire();
                let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(5));
                active.fetch_sub(1, Ordering::SeqCst);
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert!(peak.load(Ordering::SeqCst) <= 3);
    assert!(sem.try_acquire().is_some());
    println!("peak concurrency = {}", peak.load(Ordering::SeqCst));
}
