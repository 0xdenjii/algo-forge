// LeetCode 225 - Implement Stack using Queues
// Implement a LIFO stack (push, pop, top, empty) using a single FIFO queue.
// Approach: after each push, rotate the queue n-1 times so the newest element is at the front.
// Time push O(n), pop/top O(1); Space O(n).
use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack<T> { q: VecDeque<T> }

impl<T> MyStack<T> {
    pub fn new() -> Self { Self { q: VecDeque::new() } }
    pub fn push(&mut self, x: T) {
        self.q.push_back(x);
        for _ in 1..self.q.len() {
            let front = self.q.pop_front().unwrap();
            self.q.push_back(front);
        }
    }
    pub fn pop(&mut self) -> Option<T> { self.q.pop_front() }
    pub fn top(&self) -> Option<&T> { self.q.front() }
    pub fn empty(&self) -> bool { self.q.is_empty() }
}

fn main() {
    let mut s = MyStack::new();
    s.push(1); s.push(2); s.push(3);
    assert_eq!(s.top(), Some(&3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert!(!s.empty());
    println!("ok");
}
