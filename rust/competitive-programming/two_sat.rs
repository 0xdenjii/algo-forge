// Title: 2-SAT (CSES Giant Pizza)
// Problem: Given n boolean variables and clauses of the form (a OR b) over literals, find a
// satisfying assignment or report that none exists.
// Approach: Implication graph (!a -> b, !b -> a); Kosaraju SCC (iterative). Unsatisfiable iff
// x and !x share a component; otherwise x = true when comp[x] comes later in topological
// order (Kosaraju numbers components in topological order of the condensation).
// Complexity: O(n + m) time and space.
pub struct TwoSat {
    n: usize,
    g: Vec<Vec<usize>>,
    rg: Vec<Vec<usize>>,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self { n, g: vec![vec![]; 2 * n], rg: vec![vec![]; 2 * n] }
    }
    fn lit(v: usize, val: bool) -> usize {
        2 * v + (!val) as usize
    }
    /// Adds clause (var a == va) OR (var b == vb).
    pub fn add_or(&mut self, a: usize, va: bool, b: usize, vb: bool) {
        let (x, y) = (Self::lit(a, va), Self::lit(b, vb));
        for (u, v) in [(x ^ 1, y), (y ^ 1, x)] {
            self.g[u].push(v);
            self.rg[v].push(u);
        }
    }
    pub fn solve(&self) -> Option<Vec<bool>> {
        let m = 2 * self.n;
        let mut order = Vec::with_capacity(m);
        let mut vis = vec![false; m];
        for s in 0..m {
            if vis[s] {
                continue;
            }
            vis[s] = true;
            let mut st = vec![(s, 0usize)];
            while let Some((u, i)) = st.pop() {
                if i < self.g[u].len() {
                    st.push((u, i + 1));
                    let v = self.g[u][i];
                    if !vis[v] {
                        vis[v] = true;
                        st.push((v, 0));
                    }
                } else {
                    order.push(u);
                }
            }
        }
        let mut comp = vec![usize::MAX; m];
        let mut c = 0;
        for &s in order.iter().rev() {
            if comp[s] != usize::MAX {
                continue;
            }
            comp[s] = c;
            let mut st = vec![s];
            while let Some(u) = st.pop() {
                for &v in &self.rg[u] {
                    if comp[v] == usize::MAX {
                        comp[v] = c;
                        st.push(v);
                    }
                }
            }
            c += 1;
        }
        (0..self.n)
            .map(|v| {
                let (t, f) = (comp[2 * v], comp[2 * v + 1]);
                if t == f { None } else { Some(t > f) }
            })
            .collect()
    }
}

fn main() {
    let mut ts = TwoSat::new(3);
    ts.add_or(0, true, 1, true);
    ts.add_or(0, false, 2, true);
    ts.add_or(1, false, 2, false);
    let a = ts.solve().expect("satisfiable");
    assert!((a[0] || a[1]) && (!a[0] || a[2]) && (!a[1] || !a[2]));

    let mut bad = TwoSat::new(1);
    bad.add_or(0, true, 0, true);
    bad.add_or(0, false, 0, false);
    assert!(bad.solve().is_none());
    println!("ok");
}
