// LeetCode 703 - Kth Largest Element in a Stream
// Given k and an initial array, each add(val) returns the k-th largest element seen so far.
// Approach: keep a size-k min-heap of the largest values; its root is the answer.
// (std BinaryHeap is a max-heap, so wrap values in Reverse.)
// Time: O(log k) per add, O(n log k) init. Space: O(k).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut s = Self { k: k as usize, heap: BinaryHeap::with_capacity(k as usize + 1) };
        for x in nums {
            s.add(x);
        }
        s
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().map_or(i32::MIN, |r| r.0)
    }
}

fn main() {
    let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(k.add(3), 4);
    assert_eq!(k.add(5), 5);
    assert_eq!(k.add(10), 5);
    assert_eq!(k.add(9), 8);
    assert_eq!(k.add(4), 8);
    println!("kth largest ok");
}
