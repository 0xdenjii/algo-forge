// LeetCode 312 - Burst Balloons
// Bursting balloon i earns nums[i-1]*nums[i]*nums[i+1] (out of bounds = 1).
// Find the maximum coins obtainable by bursting all balloons.
// Approach: interval DP choosing the LAST balloon k burst inside open (l, r).
// Time: O(n^3), Space: O(n^2)

pub fn max_coins(nums: &[i32]) -> i32 {
    let mut a = vec![1];
    a.extend_from_slice(nums);
    a.push(1);
    let n = a.len();
    let mut dp = vec![vec![0; n]; n];
    for len in 2..n {
        for l in 0..n - len {
            let r = l + len;
            dp[l][r] = (l + 1..r)
                .map(|k| dp[l][k] + dp[k][r] + a[l] * a[k] * a[r])
                .max()
                .unwrap_or(0);
        }
    }
    dp[0][n - 1]
}

fn main() {
    assert_eq!(max_coins(&[3, 1, 5, 8]), 167);
    assert_eq!(max_coins(&[1, 5]), 10);
    println!("ok");
}
