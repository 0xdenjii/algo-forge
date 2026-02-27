// LeetCode 847 - Shortest Path Visiting All Nodes
// In an undirected connected graph of n <= 12 nodes, return the length of the shortest
// walk that visits every node (start/end anywhere, revisits allowed).
// Approach: multi-source BFS over states (node, visited-bitmask), all nodes as sources.
// Time: O(2^n * n^2), Space: O(2^n * n)
use std::collections::VecDeque;

pub fn shortest_path_length(graph: &[Vec<usize>]) -> usize {
    let n = graph.len();
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
        for &v in &graph[u] {
            let nm = mask | 1 << v;
            if !seen[v][nm] {
                seen[v][nm] = true;
                q.push_back((v, nm, d + 1));
            }
        }
    }
    0
}

fn main() {
    assert_eq!(shortest_path_length(&[vec![1, 2, 3], vec![0], vec![0], vec![0]]), 4);
    assert_eq!(shortest_path_length(&[vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]]), 4);
    println!("ok");
}
