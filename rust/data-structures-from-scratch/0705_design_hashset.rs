// LeetCode 705 - Design HashSet
// Implement add, remove, contains for integer keys without built-in hash tables.
// Approach: separate chaining over a prime number of buckets with a multiplicative hash.
// Time: O(1) average per op. Space: O(n + buckets).
const BUCKETS: usize = 769;

pub struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self { buckets: vec![Vec::new(); BUCKETS] }
    }
    fn slot(key: i32) -> usize {
        ((key as u32).wrapping_mul(2_654_435_761) as usize) % BUCKETS
    }
    pub fn add(&mut self, key: i32) {
        let b = &mut self.buckets[Self::slot(key)];
        if !b.contains(&key) {
            b.push(key);
        }
    }
    pub fn remove(&mut self, key: i32) {
        let b = &mut self.buckets[Self::slot(key)];
        if let Some(p) = b.iter().position(|&k| k == key) {
            b.swap_remove(p);
        }
    }
    pub fn contains(&self, key: i32) -> bool {
        self.buckets[Self::slot(key)].contains(&key)
    }
}

fn main() {
    let mut s = MyHashSet::new();
    s.add(1);
    s.add(2);
    assert!(s.contains(1));
    assert!(!s.contains(3));
    s.add(2);
    s.remove(2);
    assert!(!s.contains(2));
    println!("hash set ok");
}
