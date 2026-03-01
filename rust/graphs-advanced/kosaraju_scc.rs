// Kosaraju's Strongly Connected Components (classic)
// Label each vertex of a directed graph with its SCC id; ids follow the topological
// order of the condensation DAG.
// Approach: pass 1 records finish order with iterative DFS; pass 2 floods the
// transposed graph in reverse finish order.
// Time: O(V + E), Space: O(V + E)
pub fn kosaraju(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut radj = vec![Vec::new(); n];
    for u in 0..n {
        for &v in &adj[u] {
            radj[v].push(u);
        }
    }
    let mut seen = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for s in 0..n {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut st = vec![(s, 0usize)];
        while let Some((v, i)) = st.pop() {
            if i < adj[v].len() {
                st.push((v, i + 1));
                let w = adj[v][i];
                if !seen[w] {
                    seen[w] = true;
                    st.push((w, 0));
                }
            } else {
                order.push(v);
            }
        }
    }
    let mut comp = vec![usize::MAX; n];
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
    (c, comp)
}

fn main() {
    let adj = vec![vec![1], vec![2], vec![0, 3], vec![4], vec![3]];
    let (k, comp) = kosaraju(&adj);
    assert_eq!(k, 2);
    assert_eq!(comp[0], comp[2]);
    assert_eq!(comp[3], comp[4]);
    assert!(comp[0] < comp[3]);
    println!("ok");
}
