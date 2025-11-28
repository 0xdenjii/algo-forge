// LeetCode 895 - Maximum Frequency Stack
// push(x) adds x; pop() removes and returns the most frequent element, breaking ties by
// the one pushed most recently.
// Approach: freq map value -> count, and a stack per frequency level; an element with count c
// appears in levels 1..=c. Pop from the highest non-empty level.
// Time: O(1) per op. Space: O(n).
use std::collections::HashMap;

#[derive(Default)]
pub struct FreqStack {
    freq: HashMap<i32, usize>,
    levels: Vec<Vec<i32>>, // levels[f - 1] = stack of values that reached frequency f
}

impl FreqStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        if *f > self.levels.len() {
            self.levels.push(Vec::new());
        }
        self.levels[*f - 1].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let top = self.levels.last_mut().expect("empty");
        let val = top.pop().unwrap();
        if top.is_empty() {
            self.levels.pop();
        }
        let f = self.freq.get_mut(&val).unwrap();
        *f -= 1;
        if *f == 0 {
            self.freq.remove(&val);
        }
        val
    }
}

fn main() {
    let mut s = FreqStack::new();
    for x in [5, 7, 5, 7, 4, 5] {
        s.push(x);
    }
    assert_eq!(s.pop(), 5);
    assert_eq!(s.pop(), 7);
    assert_eq!(s.pop(), 5);
    assert_eq!(s.pop(), 4);
    println!("freq stack ok");
}
