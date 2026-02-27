// LeetCode 785 - Is Graph Bipartite?
// Given an undirected graph as adjacency lists (possibly disconnected), decide whether
// its vertices can be split into two sets with every edge crossing between them.
// Approach: BFS 2-coloring from every uncolored vertex; a same-color edge means odd cycle.
// Time: O(V + E), Space: O(V)
use std::collections::VecDeque;

pub fn is_bipartite(graph: &[Vec<usize>]) -> bool {
    let n = graph.len();
    let mut color: Vec<Option<bool>> = vec![None; n];
    for s in 0..n {
        if color[s].is_some() {
            continue;
        }
        color[s] = Some(false);
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            let cu = color[u].unwrap();
            for &v in &graph[u] {
                match color[v] {
                    None => {
                        color[v] = Some(!cu);
                        q.push_back(v);
                    }
                    Some(cv) if cv == cu => return false,
                    _ => {}
                }
            }
        }
    }
    true
}

fn main() {
    assert!(!is_bipartite(&[vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]));
    assert!(is_bipartite(&[vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]));
    println!("ok");
}
