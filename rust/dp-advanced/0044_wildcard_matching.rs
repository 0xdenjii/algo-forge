// LeetCode 44 - Wildcard Matching
// Match s against p where '?' matches any single char and '*' matches any
// sequence (including empty). The match must cover the whole string.
// Approach: rolling 1D DP over the pattern; dp[j] = s[..i] matches p[..j].
// Time: O(n*m), Space: O(m)

pub fn is_match(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let m = p.len();
    let mut dp = vec![false; m + 1];
    dp[0] = true;
    for j in 1..=m {
        dp[j] = dp[j - 1] && p[j - 1] == b'*';
    }
    for &c in s {
        let mut diag = dp[0];
        dp[0] = false;
        for j in 1..=m {
            let tmp = dp[j];
            dp[j] = match p[j - 1] {
                b'*' => dp[j] || dp[j - 1],
                b'?' => diag,
                x => diag && x == c,
            };
            diag = tmp;
        }
    }
    dp[m]
}

fn main() {
    assert!(!is_match("aa", "a"));
    assert!(is_match("aa", "*"));
    assert!(!is_match("cb", "?a"));
    assert!(is_match("adceb", "*a*b"));
    assert!(!is_match("acdcb", "a*c?b"));
    println!("ok");
}
