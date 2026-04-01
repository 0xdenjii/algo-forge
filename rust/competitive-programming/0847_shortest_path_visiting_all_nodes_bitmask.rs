// Problem 847: Shortest Path Visiting All Nodes
// Undirected connected graph with n <= 12 nodes. Return the length of the shortest walk that
// visits every node; it may start and end anywhere and revisit nodes and edges.
// Approach: Multi-source BFS over states (node, visited bitmask).
// Complexity: O(2^n * n^2) time, O(2^n * n) space.
use std::collections::VecDeque;

pub fn shortest_path_length(g: &[Vec<usize>]) -> usize {
    let n = g.len();
    if n <= 1 {
        return 0;
    }
    let full = (1usize << n) - 1;
    let mut seen = vec![vec![false; 1 << n]; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        seen[i][1 << i] = true;
        q.push_back((i, 1usize << i, 0usize));
    }
    while let Some((u, mask, d)) = q.pop_front() {
        if mask == full {
            return d;
        }
        for &v in &g[u] {
            let nm = mask | (1 << v);
            if !seen[v][nm] {
                seen[v][nm] = true;
                q.push_back((v, nm, d + 1));
            }
        }
    }
    unreachable!("graph is connected")
}

fn main() {
    assert_eq!(shortest_path_length(&[vec![1, 2, 3], vec![0], vec![0], vec![0]]), 4);
    assert_eq!(
        shortest_path_length(&[vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]]),
        4
    );
    println!("ok");
}
