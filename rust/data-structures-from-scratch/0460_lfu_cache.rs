// LeetCode 460 - LFU Cache
// Cache with fixed capacity; when full, evict the least frequently used key,
// breaking ties by least recently used.
// Approach: key -> (value, freq, stamp); freq -> BTreeMap<stamp, key> ordered by recency;
// track min_freq so eviction finds its bucket directly.
// Time: O(log n) per op (BTreeMap within a bucket). Space: O(capacity).
use std::collections::{BTreeMap, HashMap};

pub struct LFUCache {
    cap: usize,
    tick: u64,
    min_freq: u32,
    entries: HashMap<i32, (i32, u32, u64)>,
    buckets: HashMap<u32, BTreeMap<u64, i32>>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        LFUCache {
            cap: capacity as usize,
            tick: 0,
            min_freq: 0,
            entries: HashMap::new(),
            buckets: HashMap::new(),
        }
    }

    /// Bump the frequency of an existing key and return its value.
    fn touch(&mut self, key: i32) -> i32 {
        let (val, freq, stamp) = self.entries[&key];
        let bucket = self.buckets.get_mut(&freq).unwrap();
        bucket.remove(&stamp);
        if bucket.is_empty() {
            self.buckets.remove(&freq);
            if self.min_freq == freq {
                self.min_freq += 1;
            }
        }
        self.tick += 1;
        self.buckets.entry(freq + 1).or_default().insert(self.tick, key);
        self.entries.insert(key, (val, freq + 1, self.tick));
        val
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.entries.contains_key(&key) {
            self.touch(key)
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if self.entries.contains_key(&key) {
            self.touch(key);
            self.entries.get_mut(&key).unwrap().0 = value;
            return;
        }
        if self.entries.len() == self.cap {
            let bucket = self.buckets.get_mut(&self.min_freq).unwrap();
            let (&stamp, &victim) = bucket.iter().next().unwrap();
            bucket.remove(&stamp);
            if bucket.is_empty() {
                self.buckets.remove(&self.min_freq);
            }
            self.entries.remove(&victim);
        }
        self.tick += 1;
        self.min_freq = 1;
        self.buckets.entry(1).or_default().insert(self.tick, key);
        self.entries.insert(key, (value, 1, self.tick));
    }
}

fn main() {
    let mut c = LFUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    assert_eq!(c.get(1), 1);
    c.put(3, 3); // evicts 2
    assert_eq!(c.get(2), -1);
    assert_eq!(c.get(3), 3);
    c.put(4, 4); // 1 and 3 both freq 2; 1 is older -> evicted
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(3), 3);
    assert_eq!(c.get(4), 4);
    println!("LFU cache ok");
}
