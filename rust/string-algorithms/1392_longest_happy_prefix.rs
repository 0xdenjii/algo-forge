// LeetCode 1392 - Longest Happy Prefix
// A happy prefix is a non-empty prefix that is also a suffix (excluding s itself).
// Return the longest happy prefix of s, or "".
// Approach: the last value of the KMP prefix function is exactly that length.
// Time: O(n), Space: O(n)

pub fn longest_prefix(s: &str) -> String {
    let b = s.as_bytes();
    let mut pi = vec![0usize; b.len()];
    for i in 1..b.len() {
        let mut k = pi[i - 1];
        while k > 0 && b[i] != b[k] {
            k = pi[k - 1];
        }
        if b[i] == b[k] {
            k += 1;
        }
        pi[i] = k;
    }
    let len = pi.last().copied().unwrap_or(0);
    s[..len].to_string()
}

fn main() {
    assert_eq!(longest_prefix("level"), "l");
    assert_eq!(longest_prefix("ababab"), "abab");
    assert_eq!(longest_prefix("a"), "");
    println!("ok");
}
