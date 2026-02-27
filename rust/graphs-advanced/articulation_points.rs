// Articulation Points / Cut Vertices (classic)
// Find every vertex of an undirected graph whose removal increases the number of
// connected components.
// Approach: DFS low-link. A non-root v is a cut vertex if some DFS child c has
// low[c] >= tin[v]; the root is one iff it has 2+ DFS children.
// Time: O(V + E), Space: O(V) (recursive; fine for moderate depths)
struct State<'a> {
    adj: &'a [Vec<usize>],
    tin: Vec<usize>,
    low: Vec<usize>,
    timer: usize,
    cut: Vec<bool>,
}

impl State<'_> {
    fn dfs(&mut self, v: usize, parent: Option<usize>) {
        self.tin[v] = self.timer;
        self.low[v] = self.timer;
        self.timer += 1;
        let mut children = 0;
        for i in 0..self.adj[v].len() {
            let to = self.adj[v][i];
            if Some(to) == parent {
                continue;
            }
            if self.tin[to] != usize::MAX {
                self.low[v] = self.low[v].min(self.tin[to]);
            } else {
                self.dfs(to, Some(v));
                self.low[v] = self.low[v].min(self.low[to]);
                if parent.is_some() && self.low[to] >= self.tin[v] {
                    self.cut[v] = true;
                }
                children += 1;
            }
        }
        if parent.is_none() && children > 1 {
            self.cut[v] = true;
        }
    }
}

pub fn articulation_points(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut s = State { adj, tin: vec![usize::MAX; n], low: vec![0; n], timer: 0, cut: vec![false; n] };
    for v in 0..n {
        if s.tin[v] == usize::MAX {
            s.dfs(v, None);
        }
    }
    (0..n).filter(|&v| s.cut[v]).collect()
}

fn main() {
    // triangle 0-1-2, plus chain 1-3-4
    let adj = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1], vec![1, 4], vec![3]];
    assert_eq!(articulation_points(&adj), vec![1, 3]);
    println!("ok");
}
