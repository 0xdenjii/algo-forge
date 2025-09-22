// LeetCode 225 - Implement Stack using Queues
// Implement a LIFO stack (push, pop, top, empty) using only queue operations.
// Approach: one queue; after pushing x, rotate the previous n-1 elements behind it so x is in front.
// Time: push O(n), others O(1). Space: O(n).
use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
        for _ in 1..self.q.len() {
            let y = self.q.pop_front().unwrap();
            self.q.push_back(y);
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.q.pop_front().expect("empty stack")
    }
    pub fn top(&self) -> i32 {
        *self.q.front().expect("empty stack")
    }
    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

fn main() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.top(), 3);
    assert_eq!(s.pop(), 3);
    assert_eq!(s.pop(), 2);
    assert!(!s.empty());
    println!("stack via queues ok");
}
