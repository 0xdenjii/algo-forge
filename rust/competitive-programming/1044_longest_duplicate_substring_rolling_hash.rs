// Problem 1044: Longest Duplicate Substring
// Return any longest substring that occurs at least twice in s (occurrences may overlap),
// or "" if none exists.
// Approach: Binary search on length; for a fixed length hash every window with a polynomial
// double hash (two primes) in a HashMap and verify candidate matches byte-for-byte, so hash
// collisions can never produce a wrong answer.
// Complexity: O(n log n) expected time, O(n) space.
use std::collections::HashMap;

const M1: u64 = 1_000_000_007;
const M2: u64 = 998_244_353;
const B: u64 = 131;

fn find_dup(s: &[u8], len: usize) -> Option<usize> {
    let (mut p1, mut p2) = (1u64, 1u64);
    for _ in 0..len {
        p1 = p1 * B % M1;
        p2 = p2 * B % M2;
    }
    let (mut h1, mut h2) = (0u64, 0u64);
    let mut seen: HashMap<(u64, u64), Vec<usize>> = HashMap::new();
    for i in 0..s.len() {
        h1 = (h1 * B + s[i] as u64) % M1;
        h2 = (h2 * B + s[i] as u64) % M2;
        if i >= len {
            h1 = (h1 + M1 - s[i - len] as u64 * p1 % M1) % M1;
            h2 = (h2 + M2 - s[i - len] as u64 * p2 % M2) % M2;
        }
        if i + 1 >= len {
            let st = i + 1 - len;
            let bucket = seen.entry((h1, h2)).or_default();
            if bucket.iter().any(|&j| s[j..j + len] == s[st..st + len]) {
                return Some(st);
            }
            bucket.push(st);
        }
    }
    None
}

pub fn longest_dup_substring(s: &str) -> String {
    let b = s.as_bytes();
    let (mut lo, mut hi) = (1usize, b.len().saturating_sub(1));
    let mut best = (0, 0);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        match find_dup(b, mid) {
            Some(st) => {
                best = (st, mid);
                lo = mid + 1;
            }
            None => hi = mid - 1,
        }
    }
    s[best.0..best.0 + best.1].to_string()
}

fn main() {
    assert_eq!(longest_dup_substring("banana"), "ana");
    assert_eq!(longest_dup_substring("abcd"), "");
    assert_eq!(longest_dup_substring("aaaa"), "aaa");
    println!("ok");
}
