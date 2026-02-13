// LeetCode 516 - Longest Palindromic Subsequence
// Return the length of the longest palindromic subsequence in s.
// Approach: interval DP dp[i][j]; i iterated backwards, rolled into one row.
// Time: O(n^2), Space: O(n)

pub fn longest_palindrome_subseq(s: &str) -> usize {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![0usize; n];
    for i in (0..n).rev() {
        dp[i] = 1;
        let mut diag = 0; // dp[i+1][j-1] from previous row
        for j in i + 1..n {
            let tmp = dp[j];
            dp[j] = if s[i] == s[j] { diag + 2 } else { dp[j].max(dp[j - 1]) };
            diag = tmp;
        }
    }
    if n == 0 { 0 } else { dp[n - 1] }
}

fn main() {
    assert_eq!(longest_palindrome_subseq("bbbab"), 4);
    assert_eq!(longest_palindrome_subseq("cbbd"), 2);
    assert_eq!(longest_palindrome_subseq(""), 0);
    println!("ok");
}
