// LeetCode 242 - Valid Anagram
// Return true if t is an anagram of s (Unicode-aware follow-up included).
// Approach: count chars up for s and down for t in a HashMap; all counts must end at zero.
// Time: O(n), Space: O(k) distinct chars

use std::collections::HashMap;

pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        *cnt.entry(c).or_insert(0) -= 1;
    }
    cnt.values().all(|&v| v == 0)
}

fn main() {
    assert!(is_anagram("anagram", "nagaram"));
    assert!(!is_anagram("rat", "car"));
    assert!(is_anagram("h\u{e9}llo", "ll\u{e9}oh"));
    println!("ok");
}
