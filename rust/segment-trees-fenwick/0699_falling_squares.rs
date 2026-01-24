// LeetCode 699 - Falling Squares
// Problem: Squares [left, side] drop one at a time onto the x-axis and land on
//   top of whatever lies under [left, left + side). After each drop, report the
//   height of the tallest stack so far.
// Approach: Compress the interval endpoints. A segment tree supports range
//   "chmax" and range max. Heights never go down, so a lazy chmax tag is
//   commutative and never needs pushing down: a query just folds in the tags
//   along its path.
// Complexity: O(n log n) time, O(n) space.

struct ChmaxTree {
    mx: Vec<i64>,
    tag: Vec<i64>,
}

impl ChmaxTree {
    fn new(n: usize) -> Self {
        ChmaxTree { mx: vec![0; 4 * n.max(1)], tag: vec![0; 4 * n.max(1)] }
    }

    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, h: i64) {
        if qr < l || r < ql {
            return;
        }
        // Some cell in this node's range now reaches h.
        self.mx[node] = self.mx[node].max(h);
        if ql <= l && r <= qr {
            self.tag[node] = self.tag[node].max(h);
            return;
        }
        let m = (l + r) / 2;
        self.update(2 * node, l, m, ql, qr, h);
        self.update(2 * node + 1, m + 1, r, ql, qr, h);
    }

    fn query(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.mx[node];
        }
        let m = (l + r) / 2;
        // The tag applies to the whole node range, which overlaps the query.
        self.tag[node]
            .max(self.query(2 * node, l, m, ql, qr))
            .max(self.query(2 * node + 1, m + 1, r, ql, qr))
    }
}

fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut xs: Vec<i64> = positions
        .iter()
        .flat_map(|p| [p[0] as i64, p[0] as i64 + p[1] as i64])
        .collect();
    xs.sort_unstable();
    xs.dedup();
    let n = xs.len();
    let mut tree = ChmaxTree::new(n);
    let mut best = 0;
    let mut out = Vec::with_capacity(positions.len());
    for p in &positions {
        let (left, side) = (p[0] as i64, p[1] as i64);
        let l = xs.binary_search(&left).unwrap();
        let r = xs.binary_search(&(left + side)).unwrap() - 1; // half-open interval
        let h = tree.query(1, 0, n - 1, l, r) + side;
        tree.update(1, 0, n - 1, l, r, h);
        best = best.max(h);
        out.push(best as i32);
    }
    out
}

fn main() {
    assert_eq!(falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]), vec![2, 5, 5]);
    assert_eq!(falling_squares(vec![vec![100, 100], vec![200, 100]]), vec![100, 100]);
    assert_eq!(falling_squares(vec![vec![1, 5], vec![2, 2], vec![7, 5]]), vec![5, 7, 7]);
    println!("0699 ok");
}
