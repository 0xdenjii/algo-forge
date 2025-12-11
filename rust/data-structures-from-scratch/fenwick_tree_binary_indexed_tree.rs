// Classic - Fenwick Tree (Binary Indexed Tree)
// Maintain an array under point additions and answer prefix / range sums; also find the
// smallest index whose prefix sum reaches a target (order-statistics on counts).
// Approach: 1-indexed array where tree[i] covers (i - lowbit(i), i]; walk by lowbit for
// update and query; binary-lift over powers of two for lower_bound.
// Time: O(log n) per op. Space: O(n).
pub struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self { tree: vec![0; n + 1] }
    }

    /// O(n) construction.
    pub fn from_slice(a: &[i64]) -> Self {
        let n = a.len();
        let mut tree = vec![0; n + 1];
        tree[1..].copy_from_slice(a);
        for i in 1..=n {
            let j = i + (i & i.wrapping_neg());
            if j <= n {
                tree[j] += tree[i];
            }
        }
        Self { tree }
    }

    /// a[i] += delta (0-indexed)
    pub fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    /// sum of a[0..i)
    pub fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    /// sum of a[l..r)
    pub fn range(&self, l: usize, r: usize) -> i64 {
        self.prefix(r) - self.prefix(l)
    }

    /// Smallest 0-indexed i with prefix(i + 1) >= target, assuming non-negative values.
    pub fn lower_bound(&self, mut target: i64) -> Option<usize> {
        let n = self.tree.len() - 1;
        let mut pos = 0;
        let mut step = n.next_power_of_two();
        while step > 0 {
            if pos + step <= n && self.tree[pos + step] < target {
                pos += step;
                target -= self.tree[pos];
            }
            step /= 2;
        }
        if pos < n { Some(pos) } else { None }
    }
}

fn main() {
    let mut f = Fenwick::from_slice(&[3, 2, -1, 6, 5, 4, -3, 3, 7, 2, 3]);
    assert_eq!(f.prefix(5), 15);
    assert_eq!(f.range(2, 6), 14);
    f.add(3, 10);
    assert_eq!(f.range(2, 6), 24);

    // counts multiset: values 0..10, find k-th smallest
    let mut counts = Fenwick::new(10);
    for v in [4, 1, 7, 4, 9] {
        counts.add(v, 1);
    }
    assert_eq!(counts.lower_bound(1), Some(1));
    assert_eq!(counts.lower_bound(3), Some(4));
    assert_eq!(counts.lower_bound(5), Some(9));
    assert_eq!(counts.lower_bound(6), None);
    println!("fenwick ok");
}
