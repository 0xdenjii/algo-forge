// LeetCode 516 - Longest Palindromic Subsequence
// Return the length of the longest palindromic subsequence of s.
// Approach: interval DP dp[i][j] over s[i..=j], computed with i descending and a 1-D row.
// Time: O(n^2), Space: O(n)

pub fn longest_palindrome_subseq(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![0usize; n]; // dp[j] = answer for s[i..=j] for the current i
    for i in (0..n).rev() {
        dp[i] = 1;
        let mut diag = 0; // previous-row dp[j-1], i.e. answer for s[i+1..=j-1]
        for j in i + 1..n {
            let tmp = dp[j];
            dp[j] = if b[i] == b[j] { diag + 2 } else { dp[j].max(dp[j - 1]) };
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
