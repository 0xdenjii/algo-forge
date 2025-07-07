// LeetCode 155 - Min Stack
// Design a stack supporting push, pop, top and get_min, all in O(1).
// Approach: store (value, min-so-far) pairs so each frame knows the current minimum.
// Time O(1) per op, Space O(n).
#[derive(Default)]
pub struct MinStack { data: Vec<(i32, i32)> }

impl MinStack {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, x: i32) {
        let m = self.data.last().map_or(x, |&(_, m)| m.min(x));
        self.data.push((x, m));
    }
    pub fn pop(&mut self) -> Option<i32> { self.data.pop().map(|(v, _)| v) }
    pub fn top(&self) -> Option<i32> { self.data.last().map(|&(v, _)| v) }
    pub fn get_min(&self) -> Option<i32> { self.data.last().map(|&(_, m)| m) }
}

fn main() {
    let mut s = MinStack::new();
    s.push(-2); s.push(0); s.push(-3);
    assert_eq!(s.get_min(), Some(-3));
    s.pop();
    assert_eq!(s.top(), Some(0));
    assert_eq!(s.get_min(), Some(-2));
    println!("ok");
}
