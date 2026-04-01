// Problem 547: Number of Provinces
// Given an n x n adjacency matrix of cities, count the connected components (provinces).
// Approach: Disjoint Set Union with path halving and union by size; track component count.
// Complexity: O(n^2 * alpha(n)) time, O(n) space.
pub struct Dsu {
    p: Vec<usize>,
    sz: Vec<usize>,
    pub comps: usize,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self { p: (0..n).collect(), sz: vec![1; n], comps: n }
    }
    pub fn find(&mut self, mut x: usize) -> usize {
        while self.p[x] != x {
            self.p[x] = self.p[self.p[x]];
            x = self.p[x];
        }
        x
    }
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.sz[a] < self.sz[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        self.sz[a] += self.sz[b];
        self.comps -= 1;
        true
    }
}

pub fn find_circle_num(m: &[Vec<i32>]) -> usize {
    let mut d = Dsu::new(m.len());
    for i in 0..m.len() {
        for j in i + 1..m.len() {
            if m[i][j] == 1 {
                d.union(i, j);
            }
        }
    }
    d.comps
}

fn main() {
    assert_eq!(find_circle_num(&[vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]), 2);
    assert_eq!(find_circle_num(&[vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    println!("ok");
}
