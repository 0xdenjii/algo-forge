// LeetCode 10 - Regular Expression Matching
// Given string s and pattern p with '.' (any char) and '*' (zero or more of the
// preceding element), determine whether p matches the entire string s.
// Approach: dp[i][j] = s[i..] matches p[j..], filled bottom-up from the end.
// Time: O(n*m), Space: O(n*m)

pub fn is_match(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let (n, m) = (s.len(), p.len());
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;
    for i in (0..=n).rev() {
        for j in (0..m).rev() {
            let first = i < n && (p[j] == s[i] || p[j] == b'.');
            dp[i][j] = if j + 1 < m && p[j + 1] == b'*' {
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
