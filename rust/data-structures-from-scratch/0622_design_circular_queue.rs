// LeetCode 622 - Design Circular Queue
// Fixed-capacity ring-buffer queue: en_queue, de_queue, front, rear, is_empty, is_full.
// Approach: backing array plus head index and length; tail = (head + len) % cap.
// Time: O(1) per op. Space: O(k).
pub struct MyCircularQueue {
    buf: Vec<i32>,
    head: usize,
    len: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self { buf: vec![0; k as usize], head: 0, len: 0 }
    }
    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let idx = (self.head + self.len) % self.buf.len();
        self.buf[idx] = value;
        self.len += 1;
        true
    }
    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.buf.len();
        self.len -= 1;
        true
    }
    pub fn front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.buf[self.head] }
    }
    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buf[(self.head + self.len - 1) % self.buf.len()]
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }
}

fn main() {
    let mut q = MyCircularQueue::new(3);
    assert!(q.en_queue(1) && q.en_queue(2) && q.en_queue(3));
    assert!(!q.en_queue(4));
    assert_eq!(q.rear(), 3);
    assert!(q.is_full());
    assert!(q.de_queue());
    assert!(q.en_queue(4));
    assert_eq!(q.rear(), 4);
    assert_eq!(q.front(), 2);
    println!("circular queue ok");
}
