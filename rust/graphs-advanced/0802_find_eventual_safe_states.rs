// LeetCode 802 - Find Eventual Safe States
// A node is safe if every path starting from it ends at a terminal node. Return all
// safe nodes in ascending order.
// Approach: reverse the graph and run Kahn's algorithm from terminal nodes (out-degree
// 0); nodes whose out-degree drops to 0 are safe. Cycle members never reach 0.
// Time: O(V + E), Space: O(V + E)
use std::collections::VecDeque;

pub fn eventual_safe_nodes(graph: &[Vec<usize>]) -> Vec<usize> {
    let n = graph.len();
    let mut rev = vec![Vec::new(); n];
    let mut outdeg: Vec<usize> = graph.iter().map(Vec::len).collect();
    for (u, vs) in graph.iter().enumerate() {
        for &v in vs {
            rev[v].push(u);
        }
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&i| outdeg[i] == 0).collect();
    let mut safe = vec![false; n];
    while let Some(v) = q.pop_front() {
        safe[v] = true;
        for &u in &rev[v] {
            outdeg[u] -= 1;
            if outdeg[u] == 0 {
                q.push_back(u);
            }
        }
    }
    (0..n).filter(|&i| safe[i]).collect()
}

fn main() {
    let g = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]];
    assert_eq!(eventual_safe_nodes(&g), vec![2, 4, 5, 6]);
    println!("ok");
}
