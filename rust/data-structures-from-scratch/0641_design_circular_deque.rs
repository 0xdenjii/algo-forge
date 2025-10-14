// LeetCode 641 - Design Circular Deque
// Fixed-capacity double-ended queue: insert/delete at both ends, get_front, get_rear.
// Approach: ring buffer with head index and length; front insert moves head back modulo capacity.
// Time: O(1) per op. Space: O(k).
pub struct MyCircularDeque {
    buf: Vec<i32>,
    head: usize,
    len: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self { buf: vec![0; k as usize], head: 0, len: 0 }
    }
    fn cap(&self) -> usize {
        self.buf.len()
    }
    pub fn insert_front(&mut self, v: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.head = (self.head + self.cap() - 1) % self.cap();
        self.buf[self.head] = v;
        self.len += 1;
        true
    }
    pub fn insert_last(&mut self, v: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let i = (self.head + self.len) % self.cap();
        self.buf[i] = v;
        self.len += 1;
        true
    }
    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.cap();
        self.len -= 1;
        true
    }
    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.len -= 1;
        true
    }
    pub fn get_front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.buf[self.head] }
    }
    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buf[(self.head + self.len - 1) % self.cap()]
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn is_full(&self) -> bool {
        self.len == self.cap()
    }
}

fn main() {
    let mut d = MyCircularDeque::new(3);
    assert!(d.insert_last(1) && d.insert_last(2) && d.insert_front(3));
    assert!(!d.insert_front(4));
    assert_eq!(d.get_rear(), 2);
    assert!(d.delete_last());
    assert!(d.insert_front(4));
    assert_eq!(d.get_front(), 4);
    assert!(d.delete_front());
    assert_eq!(d.get_front(), 3);
    println!("circular deque ok");
}
