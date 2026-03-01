// Maximum Bipartite Matching - Hopcroft-Karp (classic)
// Given a bipartite graph with left part 0..L and right part 0..R, find a matching of
// maximum size.
// Approach: alternate BFS (layer free left vertices by shortest alternating paths)
// and DFS (find a maximal set of vertex-disjoint shortest augmenting paths).
// Time: O(E * sqrt(V)), Space: O(V + E)
use std::collections::VecDeque;

pub fn hopcroft_karp(adj: &[Vec<usize>], r: usize) -> (usize, Vec<Option<usize>>) {
    let l = adj.len();
    let mut match_l: Vec<Option<usize>> = vec![None; l];
    let mut match_r: Vec<Option<usize>> = vec![None; r];
    let mut dist = vec![u32::MAX; l];

    fn dfs(u: usize, adj: &[Vec<usize>], ml: &mut [Option<usize>], mr: &mut [Option<usize>], dist: &mut [u32]) -> bool {
        for &v in &adj[u] {
            let ok = match mr[v] {
                None => true,
                Some(w) => dist[w] == dist[u] + 1 && dfs(w, adj, ml, mr, dist),
            };
            if ok {
                ml[u] = Some(v);
                mr[v] = Some(u);
                return true;
            }
        }
        dist[u] = u32::MAX; // dead end: prune for this phase
        false
    }

    let mut size = 0;
    loop {
        let mut q = VecDeque::new();
        for u in 0..l {
            if match_l[u].is_none() {
                dist[u] = 0;
                q.push_back(u);
            } else {
                dist[u] = u32::MAX;
            }
        }
        let mut found = false;
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                match match_r[v] {
                    None => found = true,
                    Some(w) if dist[w] == u32::MAX => {
                        dist[w] = dist[u] + 1;
                        q.push_back(w);
                    }
                    _ => {}
                }
            }
        }
        if !found {
            break;
        }
        for u in 0..l {
            if match_l[u].is_none() && dfs(u, adj, &mut match_l, &mut match_r, &mut dist) {
                size += 1;
            }
        }
    }
    (size, match_l)
}

fn main() {
    let adj = vec![vec![0, 1], vec![0], vec![1, 2], vec![2]];
    let (sz, m) = hopcroft_karp(&adj, 3);
    assert_eq!(sz, 3);
    let used: std::collections::HashSet<_> = m.iter().flatten().collect();
    assert_eq!(used.len(), 3);
    println!("ok");
}
