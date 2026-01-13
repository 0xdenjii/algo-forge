// LeetCode 44 - Wildcard Matching
// Match the whole string s against pattern p with '?' (any single char) and '*' (any sequence).
// Approach: greedy two-pointer, backtracking to the most recent '*' on mismatch.
// Time: O(m*n) worst case, near-linear in practice; Space: O(1)

pub fn is_match(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let (mut i, mut j) = (0, 0);
    let mut star: Option<usize> = None;
    let mut mark = 0;
    while i < s.len() {
        if j < p.len() && (p[j] == b'?' || p[j] == s[i]) {
            i += 1;
            j += 1;
        } else if j < p.len() && p[j] == b'*' {
            star = Some(j);
            mark = i;
            j += 1;
        } else if let Some(st) = star {
            j = st + 1;
            mark += 1;
            i = mark;
        } else {
            return false;
        }
    }
    p[j..].iter().all(|&c| c == b'*')
}

fn main() {
    assert!(!is_match("aa", "a"));
    assert!(is_match("aa", "*"));
    assert!(!is_match("cb", "?a"));
    assert!(is_match("adceb", "*a*b"));
    assert!(!is_match("acdcb", "a*c?b"));
    println!("ok");
}
