// LeetCode 87 - Scramble String
// A string is scrambled by recursively splitting it into two non-empty parts
// and optionally swapping them. Decide whether s2 is a scramble of s1.
// Approach: interval DP dp[len][i][j] = s1[i..i+len] scrambles to s2[j..j+len].
// Time: O(n^4), Space: O(n^3)

pub fn is_scramble(s1: &str, s2: &str) -> bool {
    let (a, b) = (s1.as_bytes(), s2.as_bytes());
    let n = a.len();
    if n != b.len() {
        return false;
    }
    if n == 0 {
        return true;
    }
    let mut dp = vec![vec![vec![false; n]; n]; n + 1];
    for i in 0..n {
        for j in 0..n {
            dp[1][i][j] = a[i] == b[j];
        }
    }
    for len in 2..=n {
        for i in 0..=n - len {
            for j in 0..=n - len {
                dp[len][i][j] = (1..len).any(|k| {
                    (dp[k][i][j] && dp[len - k][i + k][j + k])
                        || (dp[k][i][j + len - k] && dp[len - k][i + k][j])
                });
            }
        }
    }
    dp[n][0][0]
}

fn main() {
    assert!(is_scramble("great", "rgeat"));
    assert!(!is_scramble("abcde", "caebd"));
    assert!(is_scramble("a", "a"));
    println!("ok");
}
