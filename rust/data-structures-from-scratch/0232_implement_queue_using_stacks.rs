// LeetCode 232 - Implement Queue using Stacks
// Implement a FIFO queue (push, pop, peek, empty) using only two LIFO stacks.
// Approach: push into `inbox`; when `outbox` is empty, pour inbox into it (reversing order).
// Time: amortized O(1) per op. Space: O(n).
#[derive(Default)]
pub struct MyQueue {
    inbox: Vec<i32>,
    outbox: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, x: i32) {
        self.inbox.push(x);
    }
    fn shift(&mut self) {
        if self.outbox.is_empty() {
            while let Some(x) = self.inbox.pop() {
                self.outbox.push(x);
            }
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.shift();
        self.outbox.pop().expect("empty queue")
    }
    pub fn peek(&mut self) -> i32 {
        self.shift();
        *self.outbox.last().expect("empty queue")
    }
    pub fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.peek(), 1);
    assert_eq!(q.pop(), 1);
    q.push(3);
    assert_eq!(q.pop(), 2);
    assert_eq!(q.pop(), 3);
    assert!(q.empty());
    println!("queue via stacks ok");
}
