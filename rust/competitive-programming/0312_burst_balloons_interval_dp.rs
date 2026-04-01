// Problem 312: Burst Balloons
// Bursting balloon i earns nums[i-1] * nums[i] * nums[i+1] (out of range counts as 1); the
// neighbours then become adjacent. Return the maximum coins obtainable.
// Approach: Interval DP choosing the LAST balloon k burst inside open interval (l, r):
// dp[l][r] = max over k of dp[l][k] + dp[k][r] + v[l] * v[k] * v[r].
// Complexity: O(n^3) time, O(n^2) space.
pub fn max_coins(nums: &[i64]) -> i64 {
    let mut v = Vec::with_capacity(nums.len() + 2);
    v.push(1);
    v.extend_from_slice(nums);
    v.push(1);
    let n = v.len();
    let mut dp = vec![vec![0i64; n]; n];
    for len in 2..n {
        for l in 0..n - len {
            let r = l + len;
            dp[l][r] = (l + 1..r)
                .map(|k| dp[l][k] + dp[k][r] + v[l] * v[k] * v[r])
                .max()
                .unwrap();
        }
    }
    dp[0][n - 1]
}

fn main() {
    assert_eq!(max_coins(&[3, 1, 5, 8]), 167);
    assert_eq!(max_coins(&[1, 5]), 10);
    assert_eq!(max_coins(&[]), 0);
    println!("ok");
}
