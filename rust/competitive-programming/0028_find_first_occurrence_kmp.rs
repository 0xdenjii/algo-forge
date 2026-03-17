// Problem 28: Find the Index of the First Occurrence in a String (KMP)
// Return the index of the first occurrence of needle in haystack, or -1. The helper also
// returns every (overlapping) occurrence.
// Approach: Prefix function pi[i] = length of the longest proper border of p[..=i]; scan the
// text, falling back through borders on mismatch.
// Complexity: O(n + m) time, O(m) space.
pub fn prefix_function(s: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; s.len()];
    for i in 1..s.len() {
        let mut k = pi[i - 1];
        while k > 0 && s[i] != s[k] {
            k = pi[k - 1];
        }
        if s[i] == s[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

pub fn find_all(text: &str, pat: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pat.as_bytes());
    if p.is_empty() {
        return vec![0];
    }
    let pi = prefix_function(p);
    let mut k = 0;
    let mut res = Vec::new();
    for (i, &c) in t.iter().enumerate() {
        while k > 0 && c != p[k] {
            k = pi[k - 1];
        }
        if c == p[k] {
            k += 1;
        }
        if k == p.len() {
            res.push(i + 1 - k);
            k = pi[k - 1];
        }
    }
    res
}

pub fn str_str(haystack: &str, needle: &str) -> i32 {
    find_all(haystack, needle).first().map_or(-1, |&i| i as i32)
}

fn main() {
    assert_eq!(prefix_function(b"aabaaab"), vec![0, 1, 0, 1, 2, 2, 3]);
    assert_eq!(find_all("sadbutsad", "sad"), vec![0, 6]);
    assert_eq!(find_all("aaaa", "aa"), vec![0, 1, 2]);
    assert_eq!(str_str("leetcode", "leeto"), -1);
    println!("ok");
}
