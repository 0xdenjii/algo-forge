// LeetCode 28 - Find the Index of the First Occurrence in a String
// Return the index of the first occurrence of needle in haystack, or None.
// Approach: Knuth-Morris-Pratt; build the prefix function of the needle, then scan once.
// Time: O(n + m), Space: O(m)

pub fn prefix_function(p: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; p.len()];
    for i in 1..p.len() {
        let mut k = pi[i - 1];
        while k > 0 && p[i] != p[k] {
            k = pi[k - 1];
        }
        if p[i] == p[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

pub fn str_str(haystack: &str, needle: &str) -> Option<usize> {
    let (h, p) = (haystack.as_bytes(), needle.as_bytes());
    if p.is_empty() {
        return Some(0);
    }
    let pi = prefix_function(p);
    let mut k = 0;
    for (i, &c) in h.iter().enumerate() {
        while k > 0 && c != p[k] {
            k = pi[k - 1];
        }
        if c == p[k] {
            k += 1;
        }
        if k == p.len() {
            return Some(i + 1 - k);
        }
    }
    None
}

fn main() {
    assert_eq!(str_str("sadbutsad", "sad"), Some(0));
    assert_eq!(str_str("leetcode", "leeto"), None);
    assert_eq!(str_str("aaaaab", "aab"), Some(3));
    println!("ok");
}
