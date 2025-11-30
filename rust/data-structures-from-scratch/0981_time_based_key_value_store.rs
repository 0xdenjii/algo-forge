// LeetCode 981 - Time Based Key-Value Store
// set(key, value, timestamp) stores a version; get(key, t) returns the value with the largest
// timestamp <= t, or "" if none. Timestamps for set are strictly increasing.
// Approach: HashMap key -> Vec<(timestamp, value)> kept sorted by construction; binary search
// with partition_point for the last timestamp <= t.
// Time: set O(1) amortized, get O(log m). Space: O(total sets).
use std::collections::HashMap;

#[derive(Default)]
pub struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        self.store
            .get(&key)
            .and_then(|v| {
                let idx = v.partition_point(|(t, _)| *t <= timestamp);
                idx.checked_sub(1).map(|i| v[i].1.clone())
            })
            .unwrap_or_default()
    }
}

fn main() {
    let mut tm = TimeMap::new();
    tm.set("foo".into(), "bar".into(), 1);
    assert_eq!(tm.get("foo".into(), 1), "bar");
    assert_eq!(tm.get("foo".into(), 3), "bar");
    tm.set("foo".into(), "bar2".into(), 4);
    assert_eq!(tm.get("foo".into(), 4), "bar2");
    assert_eq!(tm.get("foo".into(), 5), "bar2");
    assert_eq!(tm.get("foo".into(), 0), "");
    assert_eq!(tm.get("nope".into(), 9), "");
    println!("time map ok");
}
