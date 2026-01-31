// Classic - Order-Statistics Multiset on a Fenwick Tree
// Problem: Maintain a multiset of integers in [0, MAX) and support insert,
//   erase (one copy), rank(x) = #elements < x, and kth(k) = the k-th smallest
//   (1-based). This is the Codeforces 1354D "Multiset" pattern.
// Approach: A Fenwick tree of frequencies. kth uses binary lifting: walk the
//   powers of two from high to low, moving right while the accumulated count
//   stays below k. No extra log factor from a binary search.
// Complexity: O(log MAX) per operation, O(MAX) space.

struct OrderedMultiset {
    tree: Vec<i32>,
    len: usize,
    log: usize,
}

impl OrderedMultiset {
    fn new(max: usize) -> Self {
        let log = if max == 0 { 0 } else { (usize::BITS - max.leading_zeros()) as usize };
        OrderedMultiset { tree: vec![0; max + 1], len: 0, log }
    }

    fn update(&mut self, x: usize, d: i32) {
        let mut i = x + 1;
        while i < self.tree.len() {
            self.tree[i] += d;
            i += i & i.wrapping_neg();
        }
    }

    fn insert(&mut self, x: usize) {
        self.update(x, 1);
        self.len += 1;
    }

    fn erase(&mut self, x: usize) -> bool {
        if self.count(x) == 0 {
            return false;
        }
        self.update(x, -1);
        self.len -= 1;
        true
    }

    /// Number of elements strictly less than x.
    fn rank(&self, x: usize) -> i32 {
        let mut i = x.min(self.tree.len() - 1);
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i &= i - 1;
        }
        s
    }

    fn count(&self, x: usize) -> i32 {
        self.rank(x + 1) - self.rank(x)
    }

    /// k-th smallest element, 1-based.
    fn kth(&self, mut k: i32) -> Option<usize> {
        if k < 1 || k as usize > self.len {
            return None;
        }
        let mut pos = 0;
        for b in (0..self.log).rev() {
            let next = pos + (1 << b);
            if next < self.tree.len() && self.tree[next] < k {
                pos = next;
                k -= self.tree[next];
            }
        }
        Some(pos) // Fenwick index pos + 1 corresponds to value pos
    }
}

fn main() {
    let mut ms = OrderedMultiset::new(100);
    for x in [5, 1, 9, 5, 3, 99, 0] {
        ms.insert(x);
    }
    // sorted: 0 1 3 5 5 9 99
    assert_eq!(ms.kth(1), Some(0));
    assert_eq!(ms.kth(4), Some(5));
    assert_eq!(ms.kth(5), Some(5));
    assert_eq!(ms.kth(7), Some(99));
    assert_eq!(ms.kth(8), None);
    assert_eq!(ms.rank(5), 3);
    assert_eq!(ms.count(5), 2);
    assert!(ms.erase(5));
    assert!(!ms.erase(42));
    assert_eq!(ms.kth(5), Some(9));
    println!("fenwick order statistics ok");
}
