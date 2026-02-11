// LeetCode 140 - Word Break II
// Given s and a dictionary, return all sentences formed by inserting spaces
// so that every word is in the dictionary.
// Approach: memoized DFS on start index; memo[i] = all sentences of s[i..].
// Time: O(n^2 + output size), Space: O(output size)

use std::collections::{HashMap, HashSet};

pub fn word_break(s: &str, words: &[&str]) -> Vec<String> {
    let dict: HashSet<&str> = words.iter().copied().collect();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let mut memo: HashMap<usize, Vec<String>> = HashMap::new();
    let mut out = solve(s, 0, &dict, max_len, &mut memo);
    out.sort();
    out
}

fn solve(
    s: &str,
    i: usize,
    dict: &HashSet<&str>,
    max_len: usize,
    memo: &mut HashMap<usize, Vec<String>>,
) -> Vec<String> {
    if i == s.len() {
        return vec![String::new()];
    }
    if let Some(v) = memo.get(&i) {
        return v.clone();
    }
    let mut res = Vec::new();
    for j in i + 1..=s.len().min(i + max_len) {
        let w = &s[i..j];
        if dict.contains(w) {
            for rest in solve(s, j, dict, max_len, memo) {
                res.push(if rest.is_empty() { w.to_string() } else { format!("{w} {rest}") });
            }
        }
    }
    memo.insert(i, res.clone());
    res
}

fn main() {
    let r = word_break("catsanddog", &["cat", "cats", "and", "sand", "dog"]);
    assert_eq!(r, vec!["cat sand dog", "cats and dog"]);
    assert!(word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]).is_empty());
    println!("{r:?}");
}
