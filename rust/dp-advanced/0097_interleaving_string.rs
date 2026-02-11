// LeetCode 97 - Interleaving String
// Determine whether s3 is formed by interleaving s1 and s2 while preserving
// the relative order of characters within each.
// Approach: dp[j] = s3[..i+j] is an interleaving of s1[..i] and s2[..j].
// Time: O(n*m), Space: O(m)

pub fn is_interleave(s1: &str, s2: &str, s3: &str) -> bool {
    let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    if a.len() + b.len() != c.len() {
        return false;
    }
    let mut dp = vec![false; b.len() + 1];
    for i in 0..=a.len() {
        for j in 0..=b.len() {
            dp[j] = if i == 0 && j == 0 {
                true
            } else {
                (i > 0 && dp[j] && a[i - 1] == c[i + j - 1])
                    || (j > 0 && dp[j - 1] && b[j - 1] == c[i + j - 1])
            };
        }
    }
    dp[b.len()]
}

fn main() {
    assert!(is_interleave("aabcc", "dbbca", "aadbbcbcac"));
    assert!(!is_interleave("aabcc", "dbbca", "aadbbbaccc"));
    assert!(is_interleave("", "", ""));
    println!("ok");
}
