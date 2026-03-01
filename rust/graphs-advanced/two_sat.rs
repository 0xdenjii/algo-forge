// 2-SAT Satisfiability (classic)
// Given boolean variables x0..x(n-1) and clauses (a OR b) over literals, decide if a
// satisfying assignment exists and produce one.
// Approach: implication graph (!a -> b, !b -> a), SCCs via Kosaraju. Unsatisfiable iff
// x and !x share an SCC; otherwise x = true iff comp[x] comes after comp[!x] in
// topological order of the condensation.
// Time: O(n + m), Space: O(n + m)
pub struct TwoSat {
    n: usize,
    g: Vec<Vec<usize>>,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self { n, g: vec![vec![]; 2 * n] }
    }

    fn node(var: usize, val: bool) -> usize {
        2 * var + usize::from(!val)
    }

    /// Adds clause (x == vx) OR (y == vy).
    pub fn add_or(&mut self, x: usize, vx: bool, y: usize, vy: bool) {
        self.g[Self::node(x, !vx)].push(Self::node(y, vy));
        self.g[Self::node(y, !vy)].push(Self::node(x, vx));
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let m = 2 * self.n;
        let mut radj = vec![vec![]; m];
        for u in 0..m {
            for &v in &self.g[u] {
                radj[v].push(u);
            }
        }
        let mut seen = vec![false; m];
        let mut order = vec![];
        for s in 0..m {
            if seen[s] {
                continue;
            }
            seen[s] = true;
            let mut st = vec![(s, 0)];
            while let Some((v, i)) = st.pop() {
                if i < self.g[v].len() {
                    st.push((v, i + 1));
                    let w = self.g[v][i];
                    if !seen[w] {
                        seen[w] = true;
                        st.push((w, 0));
                    }
                } else {
                    order.push(v);
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
            while let Some(v) = st.pop() {
                for &w in &radj[v] {
                    if comp[w] == usize::MAX {
                        comp[w] = c;
                        st.push(w);
                    }
                }
            }
            c += 1;
        }
        (0..self.n)
            .map(|i| {
                let (t, f) = (comp[Self::node(i, true)], comp[Self::node(i, false)]);
                if t == f { None } else { Some(t > f) }
            })
            .collect()
    }
}

fn main() {
    // (x0 | x1) & (!x0 | x1) & (!x1 | x2)  -> x1 = true, x2 = true
    let mut s = TwoSat::new(3);
    s.add_or(0, true, 1, true);
    s.add_or(0, false, 1, true);
    s.add_or(1, false, 2, true);
    let a = s.solve().unwrap();
    assert!(a[1] && a[2]);
    // x0 & !x0 -> unsat
    let mut u = TwoSat::new(1);
    u.add_or(0, true, 0, true);
    u.add_or(0, false, 0, false);
    assert!(u.solve().is_none());
    println!("ok");
}
