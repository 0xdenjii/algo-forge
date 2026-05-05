// LLD - Thread-safe Token Bucket Rate Limiter
// Allow at most `rate` requests per second on average with bursts up to `capacity`,
// shared across many request-handling threads.
// Approach: lazy refill - store tokens (f64) and last refill Instant under a Mutex;
// on each try_acquire add elapsed * rate (capped), then spend one token if available.
// Time: O(1) per request. Space: O(1).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct TokenBucket {
    capacity: f64,
    rate: f64,
    st: Mutex<(f64, Instant)>,
}

impl TokenBucket {
    pub fn new(capacity: u32, rate_per_sec: f64) -> Self {
        Self { capacity: capacity as f64, rate: rate_per_sec, st: Mutex::new((capacity as f64, Instant::now())) }
    }
    pub fn try_acquire(&self) -> bool {
        let mut s = self.st.lock().unwrap();
        let now = Instant::now();
        s.0 = (s.0 + now.duration_since(s.1).as_secs_f64() * self.rate).min(self.capacity);
        s.1 = now;
        if s.0 >= 1.0 {
            s.0 -= 1.0;
            true
        } else {
            false
        }
    }
}

fn main() {
    let tb = Arc::new(TokenBucket::new(10, 50.0));
    let allowed = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let (tb, allowed) = (tb.clone(), allowed.clone());
            thread::spawn(move || {
                while start.elapsed() < Duration::from_millis(200) {
                    if tb.try_acquire() {
                        allowed.fetch_add(1, Ordering::Relaxed);
                    }
                    thread::sleep(Duration::from_micros(200));
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let n = allowed.load(Ordering::Relaxed);
    // burst 10 + ~50/s * 0.2s = ~20, allow scheduling slack
    assert!(n <= 25, "allowed {n}");
    println!("allowed {n} requests in 200ms (expected about 20)");
}
