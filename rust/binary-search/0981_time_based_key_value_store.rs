// LeetCode 981 - Time Based Key-Value Store
// Design a store with set(key, value, timestamp) and get(key, timestamp) that
// returns the value set with the largest timestamp_prev <= timestamp, or "".
// Timestamps for set are strictly increasing.
// Approach: per key, an append-only Vec<(ts, value)> sorted by ts; get uses
// partition_point for the upper bound.
// Time: set O(1) amortised, get O(log n); Space: O(total sets)

use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, key: &str, value: &str, timestamp: i32) {
        self.data
            .entry(key.to_string())
            .or_default()
            .push((timestamp, value.to_string()));
    }

    fn get(&self, key: &str, timestamp: i32) -> &str {
        let Some(v) = self.data.get(key) else { return "" };
        match v.partition_point(|(t, _)| *t <= timestamp) {
            0 => "",
            i => &v[i - 1].1,
        }
    }
}

fn main() {
    let mut tm = TimeMap::new();
    tm.set("foo", "bar", 1);
    assert_eq!(tm.get("foo", 1), "bar");
    assert_eq!(tm.get("foo", 3), "bar");
    tm.set("foo", "bar2", 4);
    assert_eq!(tm.get("foo", 4), "bar2");
    assert_eq!(tm.get("foo", 5), "bar2");
    assert_eq!(tm.get("foo", 0), "");
    assert_eq!(tm.get("baz", 9), "");
    println!("ok");
}
