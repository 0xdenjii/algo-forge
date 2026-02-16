// Classic - Travelling Salesman Problem (Held-Karp)
// Given an n x n distance matrix, find the length of the shortest tour that
// starts at city 0, visits every city exactly once, and returns to 0.
// Approach: bitmask DP dp[mask][v] = shortest path from 0 visiting mask, ending at v.
// Time: O(2^n * n^2), Space: O(2^n * n)

pub fn tsp(dist: &[Vec<u64>]) -> u64 {
    let n = dist.len();
    if n <= 1 {
        return 0;
    }
    const INF: u64 = u64::MAX / 4;
    let full = 1usize << n;
    let mut dp = vec![vec![INF; n]; full];
    dp[1][0] = 0;
    for mask in 1..full {
        if mask & 1 == 0 {
            continue;
        }
        for v in 0..n {
            let cur = dp[mask][v];
            if cur == INF {
                continue;
            }
            for u in 0..n {
                if mask & (1 << u) == 0 {
                    let nm = mask | (1 << u);
                    dp[nm][u] = dp[nm][u].min(cur + dist[v][u]);
                }
            }
        }
    }
    (1..n).map(|v| dp[full - 1][v] + dist[v][0]).min().unwrap()
}

fn main() {
    let d = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    assert_eq!(tsp(&d), 80);
    println!("ok");
}
