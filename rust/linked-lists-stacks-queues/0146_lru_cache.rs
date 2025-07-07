// LeetCode 146 - LRU Cache
// Design a cache with get/put in O(1) that evicts the least recently used key
// once capacity is exceeded.
// Approach: HashMap key -> slot index, plus a doubly linked list stored in a Vec
// arena (prev/next indices) to avoid Rc<RefCell> overhead. Head = most recent.
// Time O(1) per op, Space O(capacity).
use std::collections::HashMap;

const NIL: usize = usize::MAX;

struct Node { key: i32, val: i32, prev: usize, next: usize }

pub struct LruCache { cap: usize, map: HashMap<i32, usize>, nodes: Vec<Node>, head: usize, tail: usize }

impl LruCache {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        Self { cap, map: HashMap::with_capacity(cap), nodes: Vec::with_capacity(cap), head: NIL, tail: NIL }
    }

    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        if p != NIL { self.nodes[p].next = n; } else { self.head = n; }
        if n != NIL { self.nodes[n].prev = p; } else { self.tail = p; }
    }

    fn push_front(&mut self, i: usize) {
        self.nodes[i].prev = NIL;
        self.nodes[i].next = self.head;
        if self.head != NIL { self.nodes[self.head].prev = i; }
        self.head = i;
        if self.tail == NIL { self.tail = i; }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let &i = self.map.get(&key)?;
        self.unlink(i);
        self.push_front(i);
        Some(self.nodes[i].val)
    }

    pub fn put(&mut self, key: i32, val: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = val;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.nodes.len() < self.cap {
            self.nodes.push(Node { key, val, prev: NIL, next: NIL });
            self.nodes.len() - 1
        } else {
            let lru = self.tail;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.nodes[lru].key = key;
            self.nodes[lru].val = val;
            lru
        };
        self.map.insert(key, i);
        self.push_front(i);
    }
}

fn main() {
    let mut c = LruCache::new(2);
    c.put(1, 1); c.put(2, 2);
    assert_eq!(c.get(1), Some(1));
    c.put(3, 3); // evicts 2
    assert_eq!(c.get(2), None);
    c.put(4, 4); // evicts 1
    assert_eq!(c.get(1), None);
    assert_eq!(c.get(3), Some(3));
    assert_eq!(c.get(4), Some(4));
    println!("ok");
}
