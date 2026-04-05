// Title: Segment Tree with Lazy Propagation (range add, range sum)
// Problem: Array of n integers; process "add v to a[l..=r]" and "sum of a[l..=r]" queries
// (e.g. SPOJ HORRIBLE, CSES Range Update Queries).
// Approach: Recursive segment tree with pending-add tags pushed to children on demand.
// Complexity: O(log n) per operation, O(n) space.
pub struct SegTree {
    n: usize,
    sum: Vec<i64>,
    lz: Vec<i64>,
}

impl SegTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut s = Self { n, sum: vec![0; 4 * n.max(1)], lz: vec![0; 4 * n.max(1)] };
        if n > 0 {
            s.build(1, 0, n - 1, a);
        }
        s
    }
    fn build(&mut self, x: usize, l: usize, r: usize, a: &[i64]) {
        if l == r {
            self.sum[x] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * x, l, m, a);
        self.build(2 * x + 1, m + 1, r, a);
        self.sum[x] = self.sum[2 * x] + self.sum[2 * x + 1];
    }
    fn apply(&mut self, x: usize, len: usize, v: i64) {
        self.sum[x] += v * len as i64;
        self.lz[x] += v;
    }
    fn push(&mut self, x: usize, l: usize, m: usize, r: usize) {
        let v = std::mem::take(&mut self.lz[x]);
        if v != 0 {
            self.apply(2 * x, m - l + 1, v);
            self.apply(2 * x + 1, r - m, v);
        }
    }
    fn upd(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(x, r - l + 1, v);
            return;
        }
        let m = (l + r) / 2;
        self.push(x, l, m, r);
        self.upd(2 * x, l, m, ql, qr, v);
        self.upd(2 * x + 1, m + 1, r, ql, qr, v);
        self.sum[x] = self.sum[2 * x] + self.sum[2 * x + 1];
    }
    fn qry(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.sum[x];
        }
        let m = (l + r) / 2;
        self.push(x, l, m, r);
        self.qry(2 * x, l, m, ql, qr) + self.qry(2 * x + 1, m + 1, r, ql, qr)
    }
    pub fn add(&mut self, l: usize, r: usize, v: i64) {
        let n = self.n;
        self.upd(1, 0, n - 1, l, r, v)
    }
    pub fn sum(&mut self, l: usize, r: usize) -> i64 {
        let n = self.n;
        self.qry(1, 0, n - 1, l, r)
    }
}

fn main() {
    let mut st = SegTree::new(&[1, 2, 3, 4, 5]);
    assert_eq!(st.sum(0, 4), 15);
    st.add(1, 3, 10);
    assert_eq!(st.sum(0, 4), 45);
    assert_eq!(st.sum(2, 2), 13);
    assert_eq!(st.sum(3, 4), 19);
    println!("ok");
}
