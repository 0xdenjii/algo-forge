// LeetCode 10 - Regular Expression Matching
// Match the whole string s against pattern p supporting '.' (any char)
// and '*' (zero or more of the preceding element).
// Approach: bottom-up DP, dp[i][j] = s[i..] matches p[j..].
// Time: O(m*n), Space: O(m*n)

pub fn is_match(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let (m, n) = (s.len(), p.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[m][n] = true;
    for i in (0..=m).rev() {
        for j in (0..n).rev() {
            let first = i < m && (p[j] == s[i] || p[j] == b'.');
            dp[i][j] = if j + 1 < n && p[j + 1] == b'*' {
                dp[i][j + 2] || (first && dp[i + 1][j])
            } else {
                first && dp[i + 1][j + 1]
            };
        }
    }
    dp[0][0]
}

fn main() {
    assert!(!is_match("aa", "a"));
    assert!(is_match("aa", "a*"));
    assert!(is_match("ab", ".*"));
    assert!(is_match("aab", "c*a*b"));
    assert!(!is_match("mississippi", "mis*is*p*."));
    println!("ok");
}
