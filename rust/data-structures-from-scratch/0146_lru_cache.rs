// LeetCode 146 - LRU Cache
// Design a cache with fixed capacity supporting get(key) and put(key, value) in O(1).
// When full, put evicts the least recently used key.
// Approach: HashMap key -> slot index into an arena-backed doubly linked list (indices, no unsafe).
// Time: O(1) per op. Space: O(capacity).
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

pub struct LRUCache {
    cap: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
}

const HEAD: usize = 0;
const TAIL: usize = 1;

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let nodes = vec![
            Node { key: 0, val: 0, prev: HEAD, next: TAIL },
            Node { key: 0, val: 0, prev: HEAD, next: TAIL },
        ];
        LRUCache { cap: capacity as usize, map: HashMap::new(), nodes }
    }

    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn push_front(&mut self, i: usize) {
        let first = self.nodes[HEAD].next;
        self.nodes[i].prev = HEAD;
        self.nodes[i].next = first;
        self.nodes[first].prev = i;
        self.nodes[HEAD].next = i;
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key).copied() {
            Some(i) => {
                self.unlink(i);
                self.push_front(i);
                self.nodes[i].val
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = value;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.map.len() == self.cap {
            // Reuse the LRU node's slot.
            let lru = self.nodes[TAIL].prev;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.nodes[lru].key = key;
            self.nodes[lru].val = value;
            lru
        } else {
            self.nodes.push(Node { key, val: value, prev: HEAD, next: TAIL });
            self.nodes.len() - 1
        };
        self.push_front(i);
        self.map.insert(key, i);
    }
}

fn main() {
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    assert_eq!(c.get(1), 1);
    c.put(3, 3);
    assert_eq!(c.get(2), -1);
    c.put(4, 4);
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(3), 3);
    assert_eq!(c.get(4), 4);
    println!("LRU cache ok");
}
