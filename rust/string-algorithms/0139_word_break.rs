// LeetCode 139 - Word Break
// Given s and a dictionary, decide whether s can be segmented into dictionary words.
// Approach: DP over prefix ends; only try word lengths that actually exist in the dictionary.
// Time: O(n * distinct_lengths * L) with hashing, Space: O(n + dict)

use std::collections::HashSet;

pub fn word_break(s: &str, dict: &[&str]) -> bool {
    let words: HashSet<&str> = dict.iter().copied().collect();
    let mut lens: Vec<usize> = words.iter().map(|w| w.len()).collect();
    lens.sort_unstable();
    lens.dedup();
    let n = s.len();
    let mut ok = vec![false; n + 1];
    ok[0] = true;
    for end in 1..=n {
        ok[end] = lens
            .iter()
            .take_while(|&&l| l <= end)
            .any(|&l| ok[end - l] && s.get(end - l..end).is_some_and(|w| words.contains(w)));
    }
    ok[n]
}

fn main() {
    assert!(word_break("leetcode", &["leet", "code"]));
    assert!(word_break("applepenapple", &["apple", "pen"]));
    assert!(!word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]));
    println!("ok");
}
