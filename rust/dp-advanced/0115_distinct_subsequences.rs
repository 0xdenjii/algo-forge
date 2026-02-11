// LeetCode 115 - Distinct Subsequences
// Count the number of distinct subsequences of s that equal t.
// Approach: dp[j] = ways to form t[..j]; iterate j backwards per char of s
// so each character of s is used at most once per subsequence.
// Time: O(n*m), Space: O(m)

pub fn num_distinct(s: &str, t: &str) -> u64 {
    let t = t.as_bytes();
    let mut dp = vec![0u64; t.len() + 1];
    dp[0] = 1;
    for c in s.bytes() {
        for j in (1..=t.len()).rev() {
            if t[j - 1] == c {
                dp[j] = dp[j].wrapping_add(dp[j - 1]);
            }
        }
    }
    dp[t.len()]
}

fn main() {
    assert_eq!(num_distinct("rabbbit", "rabbit"), 3);
    assert_eq!(num_distinct("babgbag", "bag"), 5);
    println!("ok");
}
