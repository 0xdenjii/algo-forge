// LeetCode 622 - Design Circular Queue
// Implement a fixed-capacity ring buffer: en_queue, de_queue, front, rear, is_empty, is_full.
// Approach: Vec<Option<T>> with a head index and a length; tail = (head + len) % cap.
// Time O(1) per op, Space O(k).
pub struct MyCircularQueue<T> { buf: Vec<Option<T>>, head: usize, len: usize }

impl<T> MyCircularQueue<T> {
    pub fn new(k: usize) -> Self {
        Self { buf: (0..k).map(|_| None).collect(), head: 0, len: 0 }
    }
    pub fn en_queue(&mut self, x: T) -> bool {
        if self.is_full() { return false; }
        let i = (self.head + self.len) % self.buf.len();
        self.buf[i] = Some(x);
        self.len += 1;
        true
    }
    pub fn de_queue(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let v = self.buf[self.head].take();
        self.head = (self.head + 1) % self.buf.len();
        self.len -= 1;
        v
    }
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() { None } else { self.buf[self.head].as_ref() }
    }
    pub fn rear(&self) -> Option<&T> {
        if self.is_empty() { None } else { self.buf[(self.head + self.len - 1) % self.buf.len()].as_ref() }
    }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn is_full(&self) -> bool { self.len == self.buf.len() }
}

fn main() {
    let mut q = MyCircularQueue::new(3);
    assert!(q.en_queue(1) && q.en_queue(2) && q.en_queue(3));
    assert!(!q.en_queue(4));
    assert_eq!(q.rear(), Some(&3));
    assert!(q.is_full());
    assert_eq!(q.de_queue(), Some(1));
    assert!(q.en_queue(4));
    assert_eq!(q.rear(), Some(&4));
    assert_eq!(q.front(), Some(&2));
    println!("ok");
}
