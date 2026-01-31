// LeetCode 2940 - Find Building Where Alice and Bob Can Meet
// Problem: From building i a person can move to any j > i with
//   heights[j] > heights[i]. For each query (a, b), return the leftmost
//   building both Alice (at a) and Bob (at b) can reach (staying put counts),
//   or -1.
// Approach: Let a <= b. If a == b or heights[a] < heights[b], the answer is b.
//   Otherwise we need the first index j > b with heights[j] > heights[a].
//   A max segment tree answers that by descending into the leftmost subtree
//   whose max exceeds the threshold.
// Complexity: build O(n), query O(log n); space O(n).

struct MaxTree {
    n: usize,
    t: Vec<i32>,
}

impl MaxTree {
    fn new(h: &[i32]) -> Self {
        let n = h.len();
        let mut tree = MaxTree { n, t: vec![i32::MIN; 4 * n.max(1)] };
        if n > 0 {
            tree.build(1, 0, n - 1, h);
        }
        tree
    }

    fn build(&mut self, node: usize, l: usize, r: usize, h: &[i32]) {
        if l == r {
            self.t[node] = h[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * node, l, m, h);
        self.build(2 * node + 1, m + 1, r, h);
        self.t[node] = self.t[2 * node].max(self.t[2 * node + 1]);
    }

    /// Leftmost index >= from whose value is > v.
    fn first_greater(&self, node: usize, l: usize, r: usize, from: usize, v: i32) -> Option<usize> {
        if r < from || self.t[node] <= v {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let m = (l + r) / 2;
        self.first_greater(2 * node, l, m, from, v)
            .or_else(|| self.first_greater(2 * node + 1, m + 1, r, from, v))
    }
}

fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<usize>>) -> Vec<i64> {
    let tree = MaxTree::new(&heights);
    queries
        .iter()
        .map(|q| {
            let (a, b) = (q[0].min(q[1]), q[0].max(q[1]));
            if a == b || heights[a] < heights[b] {
                return b as i64;
            }
            if b + 1 >= tree.n {
                return -1;
            }
            tree.first_greater(1, 0, tree.n - 1, b + 1, heights[a])
                .map_or(-1, |j| j as i64)
        })
        .collect()
}

fn main() {
    let heights = vec![6, 4, 8, 5, 2, 7];
    let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
    assert_eq!(leftmost_building_queries(heights, queries), vec![2, 5, -1, 5, 2]);
    let heights = vec![5, 3, 8, 2, 6, 1, 4, 6];
    let queries = vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]];
    assert_eq!(leftmost_building_queries(heights, queries), vec![7, 6, -1, 4, 6]);
    println!("2940 ok");
}
