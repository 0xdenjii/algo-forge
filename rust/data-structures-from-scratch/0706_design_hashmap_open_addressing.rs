// LeetCode 706 - Design HashMap (generalized to generic keys)
// Implement put, get, remove without using a built-in hash table.
// Approach: open addressing with linear probing, tombstones for deletion, and doubling the
// table when occupied (live + tombstone) slots exceed 50%.
// Time: O(1) average per op. Space: O(n).
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
enum Slot<K, V> {
    Empty,
    Tomb,
    Full(K, V),
}

pub struct OpenMap<K, V> {
    slots: Vec<Slot<K, V>>,
    len: usize,
    used: usize, // live + tombstones
}

impl<K: Hash + Eq + Clone, V: Clone> OpenMap<K, V> {
    pub fn new() -> Self {
        Self { slots: vec![Slot::Empty; 8], len: 0, used: 0 }
    }

    fn home(&self, k: &K) -> usize {
        let mut h = DefaultHasher::new();
        k.hash(&mut h);
        (h.finish() as usize) & (self.slots.len() - 1)
    }

    fn find(&self, k: &K) -> Option<usize> {
        let mask = self.slots.len() - 1;
        let mut i = self.home(k);
        loop {
            match &self.slots[i] {
                Slot::Empty => return None,
                Slot::Full(kk, _) if kk == k => return Some(i),
                _ => i = (i + 1) & mask,
            }
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.len * 4 >= self.slots.len() { self.slots.len() * 2 } else { self.slots.len() };
        let old = std::mem::replace(&mut self.slots, vec![Slot::Empty; new_cap]);
        self.len = 0;
        self.used = 0;
        for s in old {
            if let Slot::Full(k, v) = s {
                self.put(k, v);
            }
        }
    }

    pub fn put(&mut self, k: K, v: V) {
        if let Some(i) = self.find(&k) {
            self.slots[i] = Slot::Full(k, v);
            return;
        }
        if (self.used + 1) * 2 > self.slots.len() {
            self.grow();
        }
        let mask = self.slots.len() - 1;
        let mut i = self.home(&k);
        while let Slot::Full(..) = self.slots[i] {
            i = (i + 1) & mask;
        }
        if let Slot::Empty = self.slots[i] {
            self.used += 1;
        }
        self.slots[i] = Slot::Full(k, v);
        self.len += 1;
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.find(k).and_then(|i| match &self.slots[i] {
            Slot::Full(_, v) => Some(v),
            _ => None,
        })
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let i = self.find(k)?;
        self.len -= 1;
        match std::mem::replace(&mut self.slots[i], Slot::Tomb) {
            Slot::Full(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut m = OpenMap::new();
    for i in 0..1000 {
        m.put(i, i * i);
    }
    for i in (0..1000).step_by(2) {
        assert_eq!(m.remove(&i), Some(i * i));
    }
    assert_eq!(m.len(), 500);
    assert_eq!(m.get(&3), Some(&9));
    assert_eq!(m.get(&4), None);
    m.put(3, -1);
    assert_eq!(m.get(&3), Some(&-1));
    // Churn through tombstones to exercise rehash without growth.
    for round in 0..50 {
        m.put(10_000 + round, round);
        m.remove(&(10_000 + round));
    }
    assert_eq!(m.len(), 500);
    println!("open-addressing hash map ok");
}
