// Pattern - Thread-safe Lazy Singleton (double-checked init)
// Initialise an expensive global (e.g. config) exactly once, even when many threads
// race to use it first; later accesses must be lock-free.
// Approach: std::sync::OnceLock - get_or_init runs the initialiser exactly once and
// blocks concurrent callers until it completes. A counter proves single init.
// Time: O(1) after init. Space: O(1).
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

static INIT_CALLS: AtomicUsize = AtomicUsize::new(0);
static CONFIG: OnceLock<HashMap<&'static str, String>> = OnceLock::new();

fn config() -> &'static HashMap<&'static str, String> {
    CONFIG.get_or_init(|| {
        INIT_CALLS.fetch_add(1, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(20)); // simulate expensive load
        HashMap::from([("db_url", "postgres://localhost/app".to_string()), ("pool", "8".to_string())])
    })
}

fn main() {
    let hs: Vec<_> = (0..16).map(|_| thread::spawn(|| config()["pool"].clone())).collect();
    for h in hs {
        assert_eq!(h.join().unwrap(), "8");
    }
    assert_eq!(INIT_CALLS.load(Ordering::SeqCst), 1);
    println!("initialised once; db_url = {}", config()["db_url"]);
}
