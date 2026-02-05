// Classic - Segment Tree with Lazy Assignment (range assign, range min)
// Problem: Given an array, support assign(l, r, v), which sets every element
//   of a[l..=r] to v, and min(l, r), which returns the minimum of a[l..=r].
// Approach: Each node stores its range minimum plus an Option<i64> assignment
//   tag. Assignments overwrite each other, so a newer tag simply replaces the
//   older one when pushed into a child. A node fully covered by an assignment
//   gets min = v.
// Complexity: O(log n) per operation, O(n) space.

struct AssignMinTree {
    n: usize,
    min: Vec<i64>,
    tag: Vec<Option<i64>>,
}

impl AssignMinTree {
    fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut t = AssignMinTree { n, min: vec![i64::MAX; 4 * n.max(1)], tag: vec![None; 4 * n.max(1)] };
        if n > 0 {
            t.build(1, 0, n - 1, a);
        }
        t
    }

    fn build(&mut self, node: usize, l: usize, r: usize, a: &[i64]) {
        if l == r {
            self.min[node] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * node, l, m, a);
        self.build(2 * node + 1, m + 1, r, a);
        self.min[node] = self.min[2 * node].min(self.min[2 * node + 1]);
    }

    fn apply(&mut self, node: usize, v: i64) {
        self.min[node] = v;
        self.tag[node] = Some(v);
    }

    fn push(&mut self, node: usize) {
        if let Some(v) = self.tag[node].take() {
            self.apply(2 * node, v);
            self.apply(2 * node + 1, v);
        }
    }

    fn assign_rec(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(node, v);
            return;
        }
        self.push(node);
        let m = (l + r) / 2;
        self.assign_rec(2 * node, l, m, ql, qr, v);
        self.assign_rec(2 * node + 1, m + 1, r, ql, qr, v);
        self.min[node] = self.min[2 * node].min(self.min[2 * node + 1]);
    }

    fn min_rec(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return i64::MAX;
        }
        if ql <= l && r <= qr {
            return self.min[node];
        }
        self.push(node);
        let m = (l + r) / 2;
        self.min_rec(2 * node, l, m, ql, qr).min(self.min_rec(2 * node + 1, m + 1, r, ql, qr))
    }

    fn assign(&mut self, l: usize, r: usize, v: i64) {
        self.assign_rec(1, 0, self.n - 1, l, r, v);
    }

    fn range_min(&mut self, l: usize, r: usize) -> i64 {
        self.min_rec(1, 0, self.n - 1, l, r)
    }
}

fn main() {
    let mut naive = vec![5i64, 3, 8, 6, 1, 9, 2, 7, 4, 0];
    let mut t = AssignMinTree::new(&naive);
    let ops: [(usize, usize, i64); 4] = [(0, 4, 10), (3, 8, 6), (9, 9, 11), (1, 2, -3)];
    for &(l, r, v) in &ops {
        t.assign(l, r, v);
        for x in &mut naive[l..=r] {
            *x = v;
        }
        for ql in 0..naive.len() {
            for qr in ql..naive.len() {
                assert_eq!(t.range_min(ql, qr), *naive[ql..=qr].iter().min().unwrap());
            }
        }
    }
    println!("lazy range-assign range-min ok");
}
