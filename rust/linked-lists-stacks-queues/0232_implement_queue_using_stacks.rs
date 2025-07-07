// LeetCode 232 - Implement Queue using Stacks
// Implement a FIFO queue (push, pop, peek, empty) using only two LIFO stacks.
// Approach: push onto `inbox`; when `outbox` is empty, move everything over,
// which reverses order. Each element moves at most once.
// Time amortized O(1) per op, Space O(n).
#[derive(Default)]
pub struct MyQueue<T> { inbox: Vec<T>, outbox: Vec<T> }

impl<T> MyQueue<T> {
    pub fn new() -> Self { Self { inbox: Vec::new(), outbox: Vec::new() } }
    pub fn push(&mut self, x: T) { self.inbox.push(x); }
    fn shift(&mut self) {
        if self.outbox.is_empty() {
            while let Some(x) = self.inbox.pop() { self.outbox.push(x); }
        }
    }
    pub fn pop(&mut self) -> Option<T> { self.shift(); self.outbox.pop() }
    pub fn peek(&mut self) -> Option<&T> { self.shift(); self.outbox.last() }
    pub fn empty(&self) -> bool { self.inbox.is_empty() && self.outbox.is_empty() }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1); q.push(2);
    assert_eq!(q.peek(), Some(&1));
    assert_eq!(q.pop(), Some(1));
    q.push(3);
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(3));
    assert!(q.empty());
    println!("ok");
}
