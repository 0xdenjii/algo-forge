// Classic - Binary Heap (priority queue) from scratch
// Implement a generic min-heap with push, pop, peek, and O(n) heapify from a Vec,
// then use it for heapsort.
// Approach: implicit complete binary tree in a Vec; children of i are 2i+1 and 2i+2.
// sift_up after push, sift_down after pop; heapify sifts down from the last parent.
// Time: push/pop O(log n), peek O(1), heapify O(n). Space: O(n).
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        let mut h = Self { data };
        for i in (0..h.data.len() / 2).rev() {
            h.sift_down(i);
        }
        h
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let top = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        top
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = (i - 1) / 2;
            if self.data[i] >= self.data[p] {
                break;
            }
            self.data.swap(i, p);
            i = p;
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let (l, r) = (2 * i + 1, 2 * i + 2);
            let mut m = i;
            if l < n && self.data[l] < self.data[m] {
                m = l;
            }
            if r < n && self.data[r] < self.data[m] {
                m = r;
            }
            if m == i {
                break;
            }
            self.data.swap(i, m);
            i = m;
        }
    }
}

pub fn heap_sort<T: Ord>(v: Vec<T>) -> Vec<T> {
    let mut h = MinHeap::from_vec(v);
    let mut out = Vec::with_capacity(h.len());
    while let Some(x) = h.pop() {
        out.push(x);
    }
    out
}

fn main() {
    let mut h = MinHeap::new();
    for x in [5, 3, 8, 1, 9, 2] {
        h.push(x);
    }
    assert_eq!(h.peek(), Some(&1));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(2));
    assert_eq!(h.len(), 4);

    let sorted = heap_sort(vec![9, -4, 7, 0, 7, 3, 12, -1]);
    assert_eq!(sorted, vec![-4, -1, 0, 3, 7, 7, 9, 12]);
    let words = heap_sort(vec!["pear", "apple", "fig"]);
    assert_eq!(words, vec!["apple", "fig", "pear"]);
    println!("binary heap ok");
}
