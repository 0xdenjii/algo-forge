// LeetCode 380 - Insert Delete GetRandom O(1)
// Design a set with insert, remove and get_random (uniform over current elements), all average O(1).
// Approach: Vec of values plus HashMap value -> index; remove swaps the target with the last
// element. Randomness from a small xorshift64 PRNG so the file has no external crates.
// Time: O(1) average per op. Space: O(n).
use std::collections::HashMap;

pub struct RandomizedSet {
    vals: Vec<i32>,
    pos: HashMap<i32, usize>,
    rng: u64,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self { vals: Vec::new(), pos: HashMap::new(), rng: 0x9E37_79B9_7F4A_7C15 }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.pos.contains_key(&val) {
            return false;
        }
        self.pos.insert(val, self.vals.len());
        self.vals.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        match self.pos.remove(&val) {
            None => false,
            Some(i) => {
                let last = self.vals.pop().unwrap();
                if last != val {
                    self.vals[i] = last;
                    self.pos.insert(last, i);
                }
                true
            }
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x
    }

    pub fn get_random(&mut self) -> i32 {
        assert!(!self.vals.is_empty(), "get_random on empty set");
        let i = (self.next_u64() % self.vals.len() as u64) as usize;
        self.vals[i]
    }
}

fn main() {
    let mut s = RandomizedSet::new();
    assert!(s.insert(1));
    assert!(!s.remove(2));
    assert!(s.insert(2));
    let r = s.get_random();
    assert!(r == 1 || r == 2);
    assert!(s.remove(1));
    assert!(!s.insert(2));
    assert_eq!(s.get_random(), 2);

    let mut t = RandomizedSet::new();
    for v in 0..4 {
        t.insert(v);
    }
    let mut hits = [0; 4];
    for _ in 0..40_000 {
        hits[t.get_random() as usize] += 1;
    }
    assert!(hits.iter().all(|&h| (8_000..12_000).contains(&h)));
    println!("randomized set ok: {:?}", hits);
}
