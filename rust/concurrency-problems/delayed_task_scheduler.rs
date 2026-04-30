// LLD - Delayed Task Scheduler (like ScheduledExecutorService.schedule)
// schedule(task, delay) runs the task once after the delay, on a background thread;
// tasks scheduled later but due earlier must run first. Support clean shutdown.
// Approach: min-heap of (due Instant, seq, task) under a Mutex; the worker waits on a
// Condvar with timeout until the earliest deadline, re-checking on new submissions.
// Time: O(log n) schedule / dispatch. Space: O(pending tasks).
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

type Task = Box<dyn FnOnce() + Send>;

struct Entry(Instant, u64, Task);
impl PartialEq for Entry { fn eq(&self, o: &Self) -> bool { (self.0, self.1) == (o.0, o.1) } }
impl Eq for Entry {}
impl PartialOrd for Entry { fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(o)) } }
impl Ord for Entry { fn cmp(&self, o: &Self) -> std::cmp::Ordering { (self.0, self.1).cmp(&(o.0, o.1)) } }

struct Inner { heap: BinaryHeap<Reverse<Entry>>, seq: u64, shutdown: bool }

pub struct Scheduler {
    sh: Arc<(Mutex<Inner>, Condvar)>,
    worker: Option<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let sh = Arc::new((Mutex::new(Inner { heap: BinaryHeap::new(), seq: 0, shutdown: false }), Condvar::new()));
        let s2 = sh.clone();
        let worker = thread::spawn(move || {
            let (m, cv) = &*s2;
            let mut g = m.lock().unwrap();
            loop {
                let now = Instant::now();
                match g.heap.peek() {
                    Some(Reverse(e)) if e.0 <= now => {
                        let Reverse(Entry(_, _, task)) = g.heap.pop().unwrap();
                        drop(g);
                        task();
                        g = m.lock().unwrap();
                    }
                    Some(Reverse(e)) => {
                        let wait = e.0 - now;
                        g = cv.wait_timeout(g, wait).unwrap().0;
                    }
                    None if g.shutdown => return,
                    None => g = cv.wait(g).unwrap(),
                }
            }
        });
        Self { sh, worker: Some(worker) }
    }
    pub fn schedule(&self, delay: Duration, f: impl FnOnce() + Send + 'static) {
        let (m, cv) = &*self.sh;
        let mut g = m.lock().unwrap();
        g.seq += 1;
        let seq = g.seq;
        g.heap.push(Reverse(Entry(Instant::now() + delay, seq, Box::new(f))));
        cv.notify_one();
    }
}

impl Drop for Scheduler {
    /// Runs all already-scheduled tasks, then stops.
    fn drop(&mut self) {
        self.sh.0.lock().unwrap().shutdown = true;
        self.sh.1.notify_one();
        if let Some(w) = self.worker.take() { let _ = w.join(); }
    }
}

fn main() {
    let order = Arc::new(Mutex::new(Vec::new()));
    {
        let s = Scheduler::new();
        for (name, ms) in [("c", 30u64), ("a", 10), ("b", 20)] {
            let o = order.clone();
            s.schedule(Duration::from_millis(ms), move || o.lock().unwrap().push(name));
        }
    }
    assert_eq!(*order.lock().unwrap(), vec!["a", "b", "c"]);
    println!("{:?}", order.lock().unwrap());
}
