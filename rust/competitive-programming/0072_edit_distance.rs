// Problem 72: Edit Distance
// Return the minimum number of insertions, deletions and substitutions needed to turn word1
// into word2.
// Approach: Levenshtein DP with a single rolling row and a saved diagonal value.
// Complexity: O(nm) time, O(m) space.
pub fn min_distance(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut dp: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut diag = dp[0];
        dp[0] = i;
        for j in 1..=b.len() {
            let above = dp[j];
            dp[j] = if a[i - 1] == b[j - 1] {
                diag
            } else {
                1 + diag.min(above).min(dp[j - 1])
            };
            diag = above;
        }
    }
    dp[b.len()]
}

fn main() {
    assert_eq!(min_distance("horse", "ros"), 3);
    assert_eq!(min_distance("intention", "execution"), 5);
    assert_eq!(min_distance("", "abc"), 3);
    println!("ok");
}
