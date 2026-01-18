// LeetCode 1143 - Longest Common Subsequence
// Return the length of the longest common subsequence of text1 and text2,
// and (bonus) reconstruct one such subsequence.
// Approach: classic O(m*n) DP table, then backtrack from dp[m][n].
// Time: O(m*n), Space: O(m*n) (O(min) possible if only the length is needed)

pub fn lcs(a: &str, b: &str) -> (usize, String) {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    let (mut i, mut j) = (m, n);
    let mut out = Vec::with_capacity(dp[m][n]);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            out.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    out.reverse();
    (dp[m][n], String::from_utf8(out).expect("ASCII input"))
}

fn main() {
    assert_eq!(lcs("abcde", "ace"), (3, "ace".to_string()));
    assert_eq!(lcs("abc", "def").0, 0);
    assert_eq!(lcs("AGGTAB", "GXTXAYB"), (4, "GTAB".to_string()));
    println!("ok");
}
