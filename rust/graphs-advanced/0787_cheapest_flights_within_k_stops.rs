// LeetCode 787 - Cheapest Flights Within K Stops
// Find the cheapest price from src to dst using at most k intermediate stops,
// or -1 if impossible.
// Approach: Bellman-Ford limited to k+1 rounds; each round relaxes from a snapshot of
// the previous round so paths grow by exactly one edge per round.
// Time: O(k * E), Space: O(V)
pub fn find_cheapest_price(n: usize, flights: &[[i32; 3]], src: usize, dst: usize, k: usize) -> i32 {
    const INF: i64 = i64::MAX / 4;
    let mut dist = vec![INF; n];
    dist[src] = 0;
    for _ in 0..=k {
        let prev = dist.clone();
        for &[u, v, w] in flights {
            let (u, v) = (u as usize, v as usize);
            if prev[u] < INF && prev[u] + (w as i64) < dist[v] {
                dist[v] = prev[u] + w as i64;
            }
        }
    }
    if dist[dst] >= INF { -1 } else { dist[dst] as i32 }
}

fn main() {
    let f = [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]];
    assert_eq!(find_cheapest_price(4, &f, 0, 3, 1), 700);
    assert_eq!(find_cheapest_price(3, &[[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0), 500);
    println!("ok");
}
