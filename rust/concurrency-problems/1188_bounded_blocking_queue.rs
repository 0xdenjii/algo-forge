// LeetCode 1188 - Design Bounded Blocking Queue
// Implement a thread-safe bounded queue: enqueue blocks while full, dequeue blocks
// while empty, size returns the current number of elements.
// Approach: VecDeque under a Mutex with two Condvars (not_full / not_empty).
// Time: O(1) per op. Space: O(capacity).
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct BoundedBlockingQueue<T> {
    cap: usize,
    q: Mutex<VecDeque<T>>,
    not_full: Condvar,
    not_empty: Condvar,
}

impl<T> BoundedBlockingQueue<T> {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        Self {
            cap,
            q: Mutex::new(VecDeque::with_capacity(cap)),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }
    pub fn enqueue(&self, x: T) {
        let mut q = self.not_full.wait_while(self.q.lock().unwrap(), |q| q.len() == self.cap).unwrap();
        q.push_back(x);
        self.not_empty.notify_one();
    }
    pub fn dequeue(&self) -> T {
        let mut q = self.not_empty.wait_while(self.q.lock().unwrap(), |q| q.is_empty()).unwrap();
        let x = q.pop_front().unwrap();
        self.not_full.notify_one();
        x
    }
    pub fn size(&self) -> usize {
        self.q.lock().unwrap().len()
    }
}

fn main() {
    let q = Arc::new(BoundedBlockingQueue::new(3));
    let producers: Vec<_> = (0..4u64)
        .map(|p| {
            let q = q.clone();
            thread::spawn(move || {
                for i in 0..250 {
                    q.enqueue(p * 1000 + i)
                }
            })
        })
        .collect();
    let consumers: Vec<_> = (0..2)
        .map(|_| {
            let q = q.clone();
            thread::spawn(move || (0..500).map(|_| q.dequeue()).sum::<u64>())
        })
        .collect();
    for p in producers {
        p.join().unwrap();
    }
    let total: u64 = consumers.into_iter().map(|c| c.join().unwrap()).sum();
    let expected: u64 = (0..4u64).flat_map(|p| (0..250u64).map(move |i| p * 1000 + i)).sum();
    assert_eq!(total, expected);
    println!("sum={total}, size={}", q.size());
}
