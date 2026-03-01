// Maximum Flow - Edmonds-Karp (classic)
// Compute the max s-t flow in a small directed network given as a capacity matrix,
// and recover the min cut's source side.
// Approach: Ford-Fulkerson with BFS shortest augmenting paths on the residual matrix.
// After termination, vertices reachable in the residual graph form the min cut side S.
// Time: O(V * E^2) (O(V^5) worst with a matrix), Space: O(V^2)
use std::collections::VecDeque;

pub fn edmonds_karp(mut cap: Vec<Vec<i64>>, s: usize, t: usize) -> (i64, Vec<usize>) {
    let n = cap.len();
    let mut flow = 0;
    loop {
        let mut parent = vec![usize::MAX; n];
        parent[s] = s;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for v in 0..n {
                if parent[v] == usize::MAX && cap[u][v] > 0 {
                    parent[v] = u;
                    q.push_back(v);
                }
            }
        }
        if parent[t] == usize::MAX {
            let cut = (0..n).filter(|&v| parent[v] != usize::MAX).collect();
            return (flow, cut);
        }
        let mut bottleneck = i64::MAX;
        let mut v = t;
        while v != s {
            let u = parent[v];
            bottleneck = bottleneck.min(cap[u][v]);
            v = u;
        }
        let mut v = t;
        while v != s {
            let u = parent[v];
            cap[u][v] -= bottleneck;
            cap[v][u] += bottleneck;
            v = u;
        }
        flow += bottleneck;
    }
}

fn main() {
    let mut cap = vec![vec![0i64; 4]; 4];
    cap[0][1] = 3;
    cap[0][2] = 2;
    cap[1][2] = 5;
    cap[1][3] = 2;
    cap[2][3] = 3;
    let (f, cut) = edmonds_karp(cap, 0, 3);
    assert_eq!(f, 5);
    assert!(cut.contains(&0) && !cut.contains(&3));
    println!("ok");
}
