// Problem 210: Course Schedule II
// n courses with prerequisite pairs [a, b] meaning b must precede a. Return any valid order,
// or an empty vector if impossible (the graph has a cycle).
// Approach: Kahn's algorithm: repeatedly take nodes with in-degree zero.
// Complexity: O(V + E) time and space.
use std::collections::VecDeque;

pub fn find_order(n: usize, pre: &[[usize; 2]]) -> Vec<usize> {
    let mut g = vec![Vec::new(); n];
    let mut indeg = vec![0; n];
    for &[a, b] in pre {
        g[b].push(a);
        indeg[a] += 1;
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &g[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n { order } else { Vec::new() }
}

fn main() {
    assert_eq!(find_order(2, &[[1, 0]]), vec![0, 1]);
    let o = find_order(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]);
    assert_eq!(o.len(), 4);
    assert_eq!(o[0], 0);
    assert_eq!(o[3], 3);
    assert!(find_order(2, &[[0, 1], [1, 0]]).is_empty());
    println!("ok");
}
