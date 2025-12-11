// Classic - Segment Tree with Lazy Propagation (range add, range sum)
// Support add(l, r, v) adding v to every a[i] in [l, r], and sum(l, r) over [l, r].
// Approach: recursive segment tree over 4n nodes; each node keeps its segment sum and a
// pending "add" tag that is pushed to children only when a query/update descends past it.
// Time: O(log n) per op. Space: O(n).
pub struct LazySegTree {
    n: usize,
    sum: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len().max(1);
        let mut t = Self { n, sum: vec![0; 4 * n], lazy: vec![0; 4 * n] };
        if !a.is_empty() {
            t.build(1, 0, n - 1, a);
        }
        t
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

    fn apply(&mut self, node: usize, len: usize, v: i64) {
        self.sum[node] += v * len as i64;
        self.lazy[node] += v;
    }

    fn push(&mut self, node: usize, l: usize, m: usize, r: usize) {
        let v = std::mem::take(&mut self.lazy[node]);
        if v != 0 {
            self.apply(2 * node, m - l + 1, v);
            self.apply(2 * node + 1, r - m, v);
        }
    }

    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(node, r - l + 1, v);
            return;
        }
        let m = (l + r) / 2;
        self.push(node, l, m, r);
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
        let m = (l + r) / 2;
        self.push(node, l, m, r);
        self.query(2 * node, l, m, ql, qr) + self.query(2 * node + 1, m + 1, r, ql, qr)
    }

    pub fn add(&mut self, l: usize, r: usize, v: i64) {
        self.update(1, 0, self.n - 1, l, r, v);
    }

    pub fn sum(&mut self, l: usize, r: usize) -> i64 {
        self.query(1, 0, self.n - 1, l, r)
    }
}

fn main() {
    let mut a: Vec<i64> = (1..=10).collect();
    let mut t = LazySegTree::new(&a);
    assert_eq!(t.sum(0, 9), 55);
    t.add(2, 5, 10);
    assert_eq!(t.sum(0, 9), 95);
    assert_eq!(t.sum(3, 3), 14);

    for i in 2..=5 {
        a[i] += 10;
    }
    for step in 0..500usize {
        let (x, y) = ((step * 7) % 10, (step * 3 + 1) % 10);
        let (l, r) = (x.min(y), x.max(y));
        let v = (step as i64 % 9) - 4;
        if step % 2 == 0 {
            t.add(l, r, v);
            a[l..=r].iter_mut().for_each(|e| *e += v);
        } else {
            assert_eq!(t.sum(l, r), a[l..=r].iter().sum::<i64>());
        }
    }
    println!("lazy segment tree ok");
}
