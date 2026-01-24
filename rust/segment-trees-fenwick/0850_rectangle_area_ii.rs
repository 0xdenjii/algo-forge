// LeetCode 850 - Rectangle Area II
// Problem: Given axis-aligned rectangles [x1, y1, x2, y2], return the total
//   area they cover (overlaps counted once), modulo 1e9 + 7.
// Approach: Sweep a vertical line over x. Each rectangle adds +1 coverage on
//   [y1, y2) at x1 and -1 at x2. A segment tree over the compressed
//   elementary y-intervals keeps a cover count and the covered length. There
//   is no push-down: a node with count > 0 is fully covered, otherwise its
//   length is the sum of its children.
// Complexity: O(n log n) time, O(n) space.

const MOD: i64 = 1_000_000_007;

struct CoverTree {
    ys: Vec<i64>,
    cnt: Vec<i32>,
    len: Vec<i64>,
}

impl CoverTree {
    fn new(ys: Vec<i64>) -> Self {
        let m = ys.len().saturating_sub(1).max(1);
        CoverTree { ys, cnt: vec![0; 4 * m], len: vec![0; 4 * m] }
    }

    /// Segment i is the elementary interval [ys[i], ys[i + 1]).
    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, v: i32) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.cnt[node] += v;
        } else {
            let m = (l + r) / 2;
            self.update(2 * node, l, m, ql, qr, v);
            self.update(2 * node + 1, m + 1, r, ql, qr, v);
        }
        self.len[node] = if self.cnt[node] > 0 {
            self.ys[r + 1] - self.ys[l]
        } else if l == r {
            0
        } else {
            self.len[2 * node] + self.len[2 * node + 1]
        };
    }
}

fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut ys: Vec<i64> = rectangles.iter().flat_map(|r| [r[1] as i64, r[3] as i64]).collect();
    ys.sort_unstable();
    ys.dedup();
    if ys.len() < 2 {
        return 0;
    }
    // (x, delta, y1, y2)
    let mut events: Vec<(i64, i32, i64, i64)> = Vec::with_capacity(2 * rectangles.len());
    for r in &rectangles {
        let (x1, y1, x2, y2) = (r[0] as i64, r[1] as i64, r[2] as i64, r[3] as i64);
        if x1 < x2 && y1 < y2 {
            events.push((x1, 1, y1, y2));
            events.push((x2, -1, y1, y2));
        }
    }
    events.sort_unstable();
    let segs = ys.len() - 1;
    let mut tree = CoverTree::new(ys.clone());
    let mut area: i64 = 0;
    let mut prev_x = events.first().map_or(0, |e| e.0);
    for (x, delta, y1, y2) in events {
        area = (area + (tree.len[1] % MOD) * ((x - prev_x) % MOD)) % MOD;
        prev_x = x;
        let l = ys.binary_search(&y1).unwrap();
        let r = ys.binary_search(&y2).unwrap() - 1;
        tree.update(1, 0, segs - 1, l, r, delta);
    }
    area as i32
}

fn main() {
    assert_eq!(rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]), 6);
    assert_eq!(rectangle_area(vec![vec![0, 0, 1_000_000_000, 1_000_000_000]]), 49);
    println!("0850 ok");
}
