// LLD - Fixed-size Thread Pool
// Build a pool of N worker threads that execute submitted closures, return results
// via handles, and shut down gracefully (finish queued work) when dropped.
// Approach: mpsc channel of boxed jobs shared by workers via Arc<Mutex<Receiver>>.
// Dropping the Sender closes the channel; Drop joins every worker.
// Time: O(1) submit. Space: O(queued jobs + N).
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    tx: Option<Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        let (tx, rx) = mpsc::channel::<Job>();
        let rx: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(rx));
        let workers = (0..n)
            .map(|_| {
                let rx = rx.clone();
                thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(f) => f(),
                        Err(_) => break,
                    }
                })
            })
            .collect();
        Self { tx: Some(tx), workers }
    }

    /// Submit a task and get a receiver for its result.
    pub fn submit<R, F>(&self, f: F) -> mpsc::Receiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (rtx, rrx) = mpsc::channel();
        self.tx
            .as_ref()
            .unwrap()
            .send(Box::new(move || {
                let _ = rtx.send(f());
            }))
            .expect("pool closed");
        rrx
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());
        for w in self.workers.drain(..) {
            let _ = w.join();
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    let handles: Vec<_> = (1..=10u64).map(|i| pool.submit(move || (1..=i).product::<u64>())).collect();
    let results: Vec<u64> = handles.into_iter().map(|h| h.recv().unwrap()).collect();
    assert_eq!(results[9], 3_628_800);
    println!("factorials: {results:?}");
}
