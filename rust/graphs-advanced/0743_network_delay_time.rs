// LeetCode 743 - Network Delay Time
// Given directed weighted edges (u, v, w) over nodes 1..=n and a source k, return the
// time for a signal from k to reach all nodes, or -1 if some node is unreachable.
// Approach: Dijkstra with a binary heap (Reverse for min-heap), skipping stale entries.
// Time: O((V + E) log V), Space: O(V + E)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn network_delay_time(times: &[[i32; 3]], n: usize, k: usize) -> i32 {
    let mut adj = vec![Vec::new(); n + 1];
    for &[u, v, w] in times {
        adj[u as usize].push((v as usize, w as i64));
    }
    let mut dist = vec![i64::MAX; n + 1];
    dist[k] = 0;
    let mut pq = BinaryHeap::from([Reverse((0i64, k))]);
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    let m = *dist[1..].iter().max().unwrap();
    if m == i64::MAX { -1 } else { m as i32 }
}

fn main() {
    let t = [[2, 1, 1], [2, 3, 1], [3, 4, 1]];
    assert_eq!(network_delay_time(&t, 4, 2), 2);
    assert_eq!(network_delay_time(&[[1, 2, 1]], 2, 2), -1);
    println!("ok");
}
