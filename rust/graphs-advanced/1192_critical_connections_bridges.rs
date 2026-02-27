// LeetCode 1192 - Critical Connections in a Network
// Return all bridges of an undirected graph: edges whose removal disconnects it.
// Approach: iterative Tarjan bridge-finding with discovery times and low-links; skip
// the parent *edge id* (not vertex) so parallel edges are handled correctly.
// Edge (p, v) is a bridge iff low[v] > tin[p].
// Time: O(V + E), Space: O(V + E)
pub fn critical_connections(n: usize, conns: &[[usize; 2]]) -> Vec<[usize; 2]> {
    let mut adj = vec![Vec::new(); n];
    for (id, &[a, b]) in conns.iter().enumerate() {
        adj[a].push((b, id));
        adj[b].push((a, id));
    }
    let (mut tin, mut low) = (vec![usize::MAX; n], vec![0; n]);
    let mut timer = 0;
    let mut res = Vec::new();
    for s in 0..n {
        if tin[s] != usize::MAX {
            continue;
        }
        tin[s] = timer;
        low[s] = timer;
        timer += 1;
        // (vertex, parent edge id, next neighbor index)
        let mut st = vec![(s, usize::MAX, 0usize)];
        while let Some(&(v, pe, i)) = st.last() {
            if i < adj[v].len() {
                st.last_mut().unwrap().2 += 1;
                let (to, id) = adj[v][i];
                if id == pe {
                    continue;
                }
                if tin[to] == usize::MAX {
                    tin[to] = timer;
                    low[to] = timer;
                    timer += 1;
                    st.push((to, id, 0));
                } else {
                    low[v] = low[v].min(tin[to]);
                }
            } else {
                st.pop();
                if let Some(&(p, _, _)) = st.last() {
                    low[p] = low[p].min(low[v]);
                    if low[v] > tin[p] {
                        res.push([p, v]);
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let r = critical_connections(4, &[[0, 1], [1, 2], [2, 0], [1, 3]]);
    assert_eq!(r, vec![[1, 3]]);
    assert!(critical_connections(2, &[[0, 1], [0, 1]]).is_empty());
    println!("ok");
}
