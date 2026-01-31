// Classic - Li Chao Tree (dynamic lower envelope of lines)
// Problem: Support add_line(k, b), which adds the line y = k*x + b, and
//   query(x), which returns the minimum y over all lines at an integer x in
//   [lo, hi]. This shows up in convex-hull-trick DP optimizations.
// Approach: A segment tree over x where each node keeps the line that wins
//   at its midpoint. When a new line arrives, the winner at mid stays in the
//   node and the loser is pushed into the one half where it can still win.
//   That half is chosen by comparing the two lines at the left endpoint. A
//   query takes the min over the lines on the root-to-leaf path.
// Complexity: O(log C) per insert and query; space O(C) nodes for range C.

#[derive(Clone, Copy)]
struct Line {
    k: i64,
    b: i64,
}

impl Line {
    fn at(&self, x: i64) -> i64 {
        self.k * x + self.b
    }
}

struct LiChao {
    lo: i64,
    hi: i64,
    t: Vec<Option<Line>>,
}

impl LiChao {
    fn new(lo: i64, hi: i64) -> Self {
        let n = (hi - lo + 1) as usize;
        LiChao { lo, hi, t: vec![None; 4 * n] }
    }

    fn add_line(&mut self, k: i64, b: i64) {
        self.insert(1, self.lo, self.hi, Line { k, b });
    }

    fn insert(&mut self, node: usize, l: i64, r: i64, mut line: Line) {
        let Some(mut cur) = self.t[node] else {
            self.t[node] = Some(line);
            return;
        };
        let m = l + (r - l) / 2;
        let left_better = line.at(l) < cur.at(l);
        let mid_better = line.at(m) < cur.at(m);
        if mid_better {
            std::mem::swap(&mut cur, &mut line);
        }
        self.t[node] = Some(cur);
        if l == r {
            return;
        }
        // `line` now loses at m; it can only win on the side where it won at l.
        if left_better != mid_better {
            self.insert(2 * node, l, m, line);
        } else {
            self.insert(2 * node + 1, m + 1, r, line);
        }
    }

    fn query(&self, x: i64) -> Option<i64> {
        let (mut node, mut l, mut r) = (1, self.lo, self.hi);
        let mut best: Option<i64> = None;
        loop {
            if let Some(line) = self.t[node] {
                let y = line.at(x);
                best = Some(best.map_or(y, |b| b.min(y)));
            }
            if l == r {
                return best;
            }
            let m = l + (r - l) / 2;
            if x <= m {
                node = 2 * node;
                r = m;
            } else {
                node = 2 * node + 1;
                l = m + 1;
            }
        }
    }
}

fn main() {
    let (lo, hi) = (-100, 100);
    let mut tree = LiChao::new(lo, hi);
    let lines = [(2, 3), (-1, 10), (0, -5), (5, -100), (-3, -40), (1, 0)];
    assert_eq!(tree.query(0), None);
    for (i, &(k, b)) in lines.iter().enumerate() {
        tree.add_line(k, b);
        for x in lo..=hi {
            let naive = lines[..=i].iter().map(|&(k, b)| k * x + b).min();
            assert_eq!(tree.query(x), naive);
        }
    }
    println!("Li Chao tree ok");
}
