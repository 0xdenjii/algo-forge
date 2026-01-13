// LeetCode 49 - Group Anagrams
// Group the lowercase strings that are anagrams of each other.
// Approach: key every word by its 26-letter count signature in a HashMap.
// Time: O(n * k), Space: O(n * k)

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
    for s in strs {
        let mut key = [0u8; 26];
        for b in s.bytes() {
            key[(b - b'a') as usize] += 1;
        }
        groups.entry(key).or_default().push(s);
    }
    groups.into_values().collect()
}

fn main() {
    let words = ["eat", "tea", "tan", "ate", "nat", "bat"].map(String::from).to_vec();
    let mut g: Vec<Vec<String>> = group_anagrams(words)
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    g.sort();
    assert_eq!(g, vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]);
    println!("{g:?}");
}
