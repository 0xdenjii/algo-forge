// Synchronizer - One-shot Promise / Future
// Implement a promise that a producer fulfils exactly once and a future that one or
// more consumers can block on (get) or poll (try_get) for the value.
// Approach: Arc<(Mutex<Option<T>>, Condvar)>. Promise::set consumes self so it can
// only be fulfilled once (enforced by the type system); Future is cloneable.
// Time: O(1). Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

type Cell<T> = Arc<(Mutex<Option<T>>, Condvar)>;

pub struct Promise<T>(Cell<T>);
#[derive(Clone)]
pub struct Future<T>(Cell<T>);

pub fn channel<T>() -> (Promise<T>, Future<T>) {
    let c = Arc::new((Mutex::new(None), Condvar::new()));
    (Promise(c.clone()), Future(c))
}

impl<T> Promise<T> {
    pub fn set(self, v: T) {
        *self.0 .0.lock().unwrap() = Some(v);
        self.0 .1.notify_all();
    }
}

impl<T: Clone> Future<T> {
    pub fn get(&self) -> T {
        let g = self.0 .1.wait_while(self.0 .0.lock().unwrap(), |v| v.is_none()).unwrap();
        g.clone().unwrap()
    }
    pub fn try_get(&self) -> Option<T> {
        self.0 .0.lock().unwrap().clone()
    }
}

fn main() {
    let (p, f) = channel::<String>();
    assert!(f.try_get().is_none());
    let waiters: Vec<_> = (0..3)
        .map(|i| {
            let f = f.clone();
            thread::spawn(move || format!("waiter {i} got {}", f.get()))
        })
        .collect();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        p.set("42".to_string());
    });
    for w in waiters {
        println!("{}", w.join().unwrap());
    }
    assert_eq!(f.try_get().as_deref(), Some("42"));
}
