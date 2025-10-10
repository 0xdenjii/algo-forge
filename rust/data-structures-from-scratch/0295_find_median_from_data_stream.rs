// LeetCode 295 - Find Median from Data Stream
// add_num(x) inserts numbers; find_median() returns the median of all numbers so far.
// Approach: two heaps - a max-heap `low` for the smaller half and a min-heap `high` for the
// larger half; keep low.len() == high.len() or high.len() + 1.
// Time: add O(log n), median O(1). Space: O(n).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        self.low.push(num);
        let top = self.low.pop().unwrap();
        self.high.push(Reverse(top));
        if self.high.len() > self.low.len() {
            let Reverse(x) = self.high.pop().unwrap();
            self.low.push(x);
        }
    }

    pub fn find_median(&self) -> f64 {
        let lo = *self.low.peek().expect("no numbers") as f64;
        if self.low.len() > self.high.len() {
            lo
        } else {
            (lo + self.high.peek().unwrap().0 as f64) / 2.0
        }
    }
}

fn main() {
    let mut m = MedianFinder::new();
    m.add_num(1);
    m.add_num(2);
    assert_eq!(m.find_median(), 1.5);
    m.add_num(3);
    assert_eq!(m.find_median(), 2.0);
    for x in [-5, 100, 7] {
        m.add_num(x);
    }
    // sorted: -5 1 2 3 7 100 -> (2+3)/2
    assert_eq!(m.find_median(), 2.5);
    println!("median finder ok");
}
