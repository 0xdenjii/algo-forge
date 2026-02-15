// LeetCode 1000 - Minimum Cost to Merge Stones
// Merge exactly k consecutive piles into one at a cost equal to their total.
// Return the minimum cost to end with a single pile, or -1 if impossible.
// Approach: interval DP dp[i][j] = min cost to reduce stones[i..=j] as far as
// possible; split at steps of k-1, add range sum when it collapses to one pile.
// Time: O(n^3 / k), Space: O(n^2)

pub fn merge_stones(stones: &[i32], k: usize) -> i32 {
    let n = stones.len();
    if (n - 1) % (k - 1) != 0 {
        return -1;
    }
    let mut pre = vec![0; n + 1];
    for i in 0..n {
        pre[i + 1] = pre[i] + stones[i];
    }
    let mut dp = vec![vec![0i32; n]; n];
    for len in k..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = (i..j)
                .step_by(k - 1)
                .map(|m| dp[i][m] + dp[m + 1][j])
                .min()
                .unwrap();
            if (len - 1) % (k - 1) == 0 {
                dp[i][j] += pre[j + 1] - pre[i];
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    assert_eq!(merge_stones(&[3, 2, 4, 1], 2), 20);
    assert_eq!(merge_stones(&[3, 2, 4, 1], 3), -1);
    assert_eq!(merge_stones(&[3, 5, 1, 2, 6], 3), 25);
    println!("ok");
}
