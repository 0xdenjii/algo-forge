// LeetCode 664 - Strange Printer
// A printer prints runs of a single character over any range, overwriting.
// Return the minimum number of turns needed to print s.
// Approach: interval DP: dp[i][j] = dp[i][j-1] + 1, improved when s[k] == s[j]
// lets the last char share a turn: dp[i][k] + dp[k+1][j-1].
// Time: O(n^3), Space: O(n^2)

pub fn strange_printer(s: &str) -> usize {
    let mut b = s.as_bytes().to_vec();
    b.dedup();
    let n = b.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0usize; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in i + 1..n {
            let mut best = dp[i][j - 1] + 1;
            for k in i..j {
                if b[k] == b[j] {
                    let right = if k + 1 <= j - 1 { dp[k + 1][j - 1] } else { 0 };
                    best = best.min(dp[i][k] + right);
                }
            }
            dp[i][j] = best;
        }
    }
    dp[0][n - 1]
}

fn main() {
    assert_eq!(strange_printer("aaabbb"), 2);
    assert_eq!(strange_printer("aba"), 2);
    assert_eq!(strange_printer("tbgtgb"), 4);
    println!("ok");
}
