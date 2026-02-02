// Classic - Segment Tree with Lazy Propagation (range add, range sum)
// Problem: Given an array, support add(l, r, v), which adds v to every
//   element of a[l..=r], and sum(l, r). (SPOJ HORRIBLE / CSES "Range Update
//   Queries" style.)
// Approach: A recursive segment tree. Each node stores its range sum plus a
//   pending add tag. A node fully inside the update range takes the tag in
//   O(1); a partial overlap pushes the tag down to the children before
//   recursing.
// Complexity: O(log n) per operation, O(n) space.

struct LazySegTree {
    n: usize,
    sum: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegTree {
    fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = LazySegTree { n, sum: vec![0; 4 * n.max(1)], lazy: vec![0; 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1, a);
        }
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, a: &[i64]) {
        if l == r {
            self.sum[node] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * node, l, m, a);
        self.build(2 * node + 1, m + 1, r, a);
        self.sum[node] = self.sum[2 * node] + self.sum[2 * node + 1];
    }

    fn apply(&mut self, node: usize, l: usize, r: usize, v: i64) {
        self.sum[node] += v * (r - l + 1) as i64;
        self.lazy[node] += v;
    }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        let v = std::mem::take(&mut self.lazy[node]);
        if v != 0 {
            let m = (l + r) / 2;
            self.apply(2 * node, l, m, v);
            self.apply(2 * node + 1, m + 1, r, v);
        }
    }

    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(node, l, r, v);
            return;
        }
        self.push(node, l, r);
        let m = (l + r) / 2;
        self.update(2 * node, l, m, ql, qr, v);
        self.update(2 * node + 1, m + 1, r, ql, qr, v);
        self.sum[node] = self.sum[2 * node] + self.sum[2 * node + 1];
    }

    fn query(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.sum[node];
        }
        self.push(node, l, r);
        let m = (l + r) / 2;
        self.query(2 * node, l, m, ql, qr) + self.query(2 * node + 1, m + 1, r, ql, qr)
    }

    fn add(&mut self, l: usize, r: usize, v: i64) {
        self.update(1, 0, self.n - 1, l, r, v);
    }

    fn range_sum(&mut self, l: usize, r: usize) -> i64 {
        self.query(1, 0, self.n - 1, l, r)
    }
}

fn main() {
    let base = vec![1i64, 2, 3, 4, 5, 6, 7, 8];
    let mut st = LazySegTree::new(&base);
    let mut naive = base.clone();
    assert_eq!(st.range_sum(0, 7), 36);
    st.add(2, 5, 10);
    for x in &mut naive[2..=5] {
        *x += 10;
    }
    st.add(0, 3, -4);
    for x in &mut naive[0..=3] {
        *x -= 4;
    }
    for l in 0..8 {
        for r in l..8 {
            assert_eq!(st.range_sum(l, r), naive[l..=r].iter().sum::<i64>());
        }
    }
    println!("lazy range-add range-sum ok");
}
