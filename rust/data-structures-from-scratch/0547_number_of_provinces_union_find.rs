// LeetCode 547 - Number of Provinces (with a reusable Disjoint Set Union)
// Given an n x n adjacency matrix of cities, count connected components (provinces).
// Approach: union-find with path compression (path halving) and union by size;
// each successful union reduces the component count by one.
// Time: O(n^2 * alpha(n)). Space: O(n).
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), size: vec![1; n], components: n }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
        true
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn components(&self) -> usize {
        self.components
    }
}

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut d = Dsu::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if is_connected[i][j] == 1 {
                d.union(i, j);
            }
        }
    }
    d.components() as i32
}

fn main() {
    assert_eq!(find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]), 2);
    assert_eq!(find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    let mut d = Dsu::new(5);
    d.union(0, 4);
    d.union(4, 2);
    assert!(d.same(0, 2));
    assert!(!d.same(1, 3));
    println!("union find ok");
}
