// LeetCode 146 (concurrent variant) - Thread-safe LRU Cache
// Implement get/put with O(1) LRU eviction that may be called from many threads.
// Approach: index-based doubly linked list in a Vec (no unsafe, no Rc) plus a
// HashMap key -> slot; the whole cache sits behind one Mutex because every get
// mutates recency order. Free slots are recycled.
// Time: O(1) average per op. Space: O(capacity).
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread;

const NIL: usize = usize::MAX;

struct Node<K, V> { k: K, v: V, prev: usize, next: usize }

struct Lru<K, V> {
    cap: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: usize, // most recent
    tail: usize, // least recent
}

impl<K: Hash + Eq + Clone, V: Clone> Lru<K, V> {
    fn new(cap: usize) -> Self {
        assert!(cap > 0);
        Self { cap, map: HashMap::new(), nodes: Vec::with_capacity(cap), head: NIL, tail: NIL }
    }
    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        if p != NIL { self.nodes[p].next = n } else { self.head = n }
        if n != NIL { self.nodes[n].prev = p } else { self.tail = p }
    }
    fn push_front(&mut self, i: usize) {
        self.nodes[i].prev = NIL;
        self.nodes[i].next = self.head;
        if self.head != NIL { self.nodes[self.head].prev = i }
        self.head = i;
        if self.tail == NIL { self.tail = i }
    }
    fn get(&mut self, k: &K) -> Option<V> {
        let i = *self.map.get(k)?;
        self.unlink(i);
        self.push_front(i);
        Some(self.nodes[i].v.clone())
    }
    fn put(&mut self, k: K, v: V) {
        if let Some(&i) = self.map.get(&k) {
            self.nodes[i].v = v;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.nodes.len() < self.cap {
            self.nodes.push(Node { k: k.clone(), v, prev: NIL, next: NIL });
            self.nodes.len() - 1
        } else {
            let t = self.tail;
            self.unlink(t);
            let old = std::mem::replace(&mut self.nodes[t].k, k.clone());
            self.map.remove(&old);
            self.nodes[t].v = v;
            t
        };
        self.map.insert(k, i);
        self.push_front(i);
    }
}

pub struct ConcurrentLru<K, V>(Mutex<Lru<K, V>>);

impl<K: Hash + Eq + Clone, V: Clone> ConcurrentLru<K, V> {
    pub fn new(cap: usize) -> Self { Self(Mutex::new(Lru::new(cap))) }
    pub fn get(&self, k: &K) -> Option<V> { self.0.lock().unwrap().get(k) }
    pub fn put(&self, k: K, v: V) { self.0.lock().unwrap().put(k, v) }
}

fn main() {
    // LeetCode example, single-threaded semantics
    let c = ConcurrentLru::new(2);
    c.put(1, 1); c.put(2, 2);
    assert_eq!(c.get(&1), Some(1));
    c.put(3, 3);
    assert_eq!(c.get(&2), None);
    c.put(4, 4);
    assert_eq!(c.get(&1), None);
    assert_eq!(c.get(&3), Some(3));
    assert_eq!(c.get(&4), Some(4));

    // concurrent hammering
    let c = Arc::new(ConcurrentLru::new(64));
    let hs: Vec<_> = (0..8u64).map(|t| {
        let c = c.clone();
        thread::spawn(move || for i in 0..5000u64 {
            let k = (i * 31 + t) % 128;
            if let Some(v) = c.get(&k) { assert_eq!(v, k * 10) } else { c.put(k, k * 10) }
        })
    }).collect();
    for h in hs { h.join().unwrap(); }
    println!("LRU ok");
}
