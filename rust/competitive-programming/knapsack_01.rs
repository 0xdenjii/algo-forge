// Title: 0/1 Knapsack (AtCoder Educational DP Contest, Problem D)
// Problem: n items with weight w_i and value v_i, capacity W. Maximize total value using each
// item at most once.
// Approach: dp[c] = best value with capacity c; iterate c downward so each item is used once.
// Complexity: O(nW) time, O(W) space.
pub fn knapsack(items: &[(usize, u64)], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &(w, v) in items {
        for c in (w..=cap).rev() {
            dp[c] = dp[c].max(dp[c - w] + v);
        }
    }
    dp[cap]
}

fn main() {
    assert_eq!(knapsack(&[(3, 30), (4, 50), (5, 60)], 8), 90);
    assert_eq!(knapsack(&[(6, 5), (5, 6), (6, 4), (6, 6), (3, 5), (7, 2)], 15), 17);
    assert_eq!(knapsack(&[(10, 100)], 5), 0);
    println!("ok");
}
