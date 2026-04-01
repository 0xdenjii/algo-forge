// Problem 743: Network Delay Time
// Directed weighted graph with n nodes (1-indexed); a signal starts at node k. Return the
// time for all nodes to receive it, or -1 if some node is unreachable.
// Approach: Dijkstra with a BinaryHeap of Reverse((dist, node)) and lazy deletion.
// Complexity: O((V + E) log V) time, O(V + E) space.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn network_delay_time(times: &[[usize; 3]], n: usize, k: usize) -> i64 {
    let mut g = vec![Vec::new(); n + 1];
    for &[u, v, w] in times {
        g[u].push((v, w as u64));
    }
    let mut dist = vec![u64::MAX; n + 1];
    let mut pq = BinaryHeap::new();
    dist[k] = 0;
    pq.push(Reverse((0u64, k)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &g[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    let m = dist[1..].iter().copied().max().unwrap_or(0);
    if m == u64::MAX { -1 } else { m as i64 }
}

fn main() {
    assert_eq!(network_delay_time(&[[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2), 2);
    assert_eq!(network_delay_time(&[[1, 2, 1]], 2, 1), 1);
    assert_eq!(network_delay_time(&[[1, 2, 1]], 2, 2), -1);
    println!("ok");
}
