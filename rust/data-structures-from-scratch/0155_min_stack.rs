// LeetCode 155 - Min Stack
// Design a stack supporting push, pop, top and get_min, all in O(1).
// Approach: store (value, min_so_far) pairs so each frame remembers the minimum beneath it.
// Time: O(1) per op. Space: O(n).
#[derive(Default)]
pub struct MinStack {
    data: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, val: i32) {
        let m = self.data.last().map_or(val, |&(_, m)| m.min(val));
        self.data.push((val, m));
    }
    pub fn pop(&mut self) {
        self.data.pop();
    }
    pub fn top(&self) -> i32 {
        self.data.last().expect("empty stack").0
    }
    pub fn get_min(&self) -> i32 {
        self.data.last().expect("empty stack").1
    }
}

fn main() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.get_min(), -3);
    s.pop();
    assert_eq!(s.top(), 0);
    assert_eq!(s.get_min(), -2);
    println!("min stack ok");
}
