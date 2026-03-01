// Kruskal's Minimum Spanning Tree (classic)
// Given an undirected weighted graph, return the MST weight and its edges,
// or None if the graph is disconnected.
// Approach: sort edges by weight; union-find with path compression + union by size.
// Time: O(E log E), Space: O(V)
struct Dsu {
    p: Vec<usize>,
    sz: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self { p: (0..n).collect(), sz: vec![1; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.p[r] != r {
            r = self.p[r];
        }
        let mut c = x;
        while self.p[c] != r {
            let nx = self.p[c];
            self.p[c] = r;
            c = nx;
        }
        r
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.sz[a] < self.sz[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        self.sz[a] += self.sz[b];
        true
    }
}

pub fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> Option<(i64, Vec<(usize, usize, i64)>)> {
    edges.sort_by_key(|e| e.2);
    let mut dsu = Dsu::new(n);
    let mut chosen = Vec::with_capacity(n.saturating_sub(1));
    let mut w = 0;
    for (u, v, c) in edges {
        if dsu.union(u, v) {
            w += c;
            chosen.push((u, v, c));
        }
    }
    if n == 0 || chosen.len() + 1 == n { Some((w, chosen)) } else { None }
}

fn main() {
    let e = vec![(0, 1, 4), (0, 2, 3), (1, 2, 1), (1, 3, 2), (2, 3, 4), (3, 4, 2), (4, 5, 6)];
    let (w, es) = kruskal(6, e).unwrap();
    assert_eq!(w, 14);
    assert_eq!(es.len(), 5);
    assert!(kruskal(3, vec![(0, 1, 1)]).is_none());
    println!("ok");
}
