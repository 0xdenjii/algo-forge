// LeetCode 210 - Course Schedule II
// Given n courses and prerequisite pairs [a, b] (take b before a), return a valid
// ordering of all courses, or an empty vector if a cycle makes it impossible.
// Approach: Kahn's algorithm (BFS over in-degree-zero vertices).
// Time: O(V + E), Space: O(V + E)
use std::collections::VecDeque;

pub fn find_order(n: usize, prereq: &[[usize; 2]]) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    let mut indeg = vec![0; n];
    for &[a, b] in prereq {
        adj[b].push(a);
        indeg[a] += 1;
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n { order } else { Vec::new() }
}

fn main() {
    let o = find_order(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]);
    assert_eq!(o.len(), 4);
    assert_eq!(o[0], 0);
    assert_eq!(*o.last().unwrap(), 3);
    assert!(find_order(2, &[[0, 1], [1, 0]]).is_empty());
    println!("ok");
}
