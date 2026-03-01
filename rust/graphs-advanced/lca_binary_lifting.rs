// Lowest Common Ancestor - Binary Lifting (classic; cf. LeetCode 236 on general trees)
// Preprocess a rooted tree so that LCA(u, v) and dist(u, v) queries run fast.
// Approach: BFS for depth and parent; up[k][v] = 2^k-th ancestor. Lift the deeper
// node to equal depth, then lift both while ancestors differ.
// Time: O(n log n) build, O(log n) per query; Space: O(n log n)
use std::collections::VecDeque;

pub struct Lca {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Lca {
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let log = (usize::BITS - n.max(1).leading_zeros()) as usize;
        let mut depth = vec![0; n];
        let mut parent = vec![root; n];
        let mut seen = vec![false; n];
        seen[root] = true;
        let mut q = VecDeque::from([root]);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    parent[v] = u;
                    depth[v] = depth[u] + 1;
                    q.push_back(v);
                }
            }
        }
        let mut up = vec![parent];
        for k in 1..log.max(1) {
            let prev = &up[k - 1];
            let next: Vec<usize> = (0..n).map(|v| prev[prev[v]]).collect();
            up.push(next);
        }
        Self { up, depth }
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let diff = self.depth[a] - self.depth[b];
        for (k, row) in self.up.iter().enumerate() {
            if diff >> k & 1 == 1 {
                a = row[a];
            }
        }
        if a == b {
            return a;
        }
        for row in self.up.iter().rev() {
            if row[a] != row[b] {
                a = row[a];
                b = row[b];
            }
        }
        self.up[0][a]
    }

    pub fn dist(&self, a: usize, b: usize) -> usize {
        self.depth[a] + self.depth[b] - 2 * self.depth[self.lca(a, b)]
    }
}

fn main() {
    //        0
    //      /   \
    //     1     2
    //    / \     \
    //   3   4     5
    //       |
    //       6
    let edges = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (4, 6)];
    let mut adj = vec![vec![]; 7];
    for &(a, b) in &edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let l = Lca::new(&adj, 0);
    assert_eq!(l.lca(3, 6), 1);
    assert_eq!(l.lca(6, 5), 0);
    assert_eq!(l.lca(4, 6), 4);
    assert_eq!(l.dist(3, 5), 4);
    println!("ok");
}
