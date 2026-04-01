// Problem 1192: Critical Connections in a Network
// Undirected graph of n servers. Return every edge whose removal disconnects some pair of
// servers (the bridges of the graph).
// Approach: DFS with discovery time tin and low-link; tree edge (p, u) is a bridge iff
// low[u] > tin[p]. Iterative DFS skips the parent by edge id, so parallel edges are handled.
// Complexity: O(V + E) time and space.
pub fn critical_connections(n: usize, conns: &[[usize; 2]]) -> Vec<[usize; 2]> {
    let mut g = vec![Vec::new(); n];
    for (id, &[a, b]) in conns.iter().enumerate() {
        g[a].push((b, id));
        g[b].push((a, id));
    }
    let mut tin = vec![usize::MAX; n];
    let mut low = vec![0; n];
    let mut timer = 0;
    let mut res = Vec::new();
    for s in 0..n {
        if tin[s] != usize::MAX {
            continue;
        }
        // frame: (node, parent edge id, next adjacency index)
        let mut st: Vec<(usize, usize, usize)> = vec![(s, usize::MAX, 0)];
        tin[s] = timer;
        low[s] = timer;
        timer += 1;
        while let Some(top) = st.last_mut() {
            let (u, pe, i) = *top;
            if i < g[u].len() {
                top.2 += 1;
                let (v, id) = g[u][i];
                if id == pe {
                    continue;
                }
                if tin[v] == usize::MAX {
                    tin[v] = timer;
                    low[v] = timer;
                    timer += 1;
                    st.push((v, id, 0));
                } else {
                    low[u] = low[u].min(tin[v]);
                }
            } else {
                st.pop();
                if let Some(&(p, _, _)) = st.last() {
                    low[p] = low[p].min(low[u]);
                    if low[u] > tin[p] {
                        res.push([p, u]);
                    }
                }
            }
        }
    }
    res
}

fn main() {
    assert_eq!(critical_connections(4, &[[0, 1], [1, 2], [2, 0], [1, 3]]), vec![[1, 3]]);
    assert_eq!(critical_connections(2, &[[0, 1]]), vec![[0, 1]]);
    assert!(critical_connections(2, &[[0, 1], [0, 1]]).is_empty());
    println!("ok");
}
