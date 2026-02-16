// LeetCode 269 - Alien Dictionary
// Words are sorted lexicographically in an unknown alphabet. Derive a valid letter
// order, or "" if inconsistent (a cycle, or a word precedes its own proper prefix).
// Approach: the first differing letter of adjacent words gives an edge; Kahn topo sort.
// Time: O(total characters), Space: O(alphabet^2)
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn alien_order(words: &[&str]) -> String {
    let mut adj: BTreeMap<u8, BTreeSet<u8>> = BTreeMap::new();
    let mut indeg: BTreeMap<u8, usize> = BTreeMap::new();
    for w in words {
        for c in w.bytes() {
            adj.entry(c).or_default();
            indeg.entry(c).or_insert(0);
        }
    }
    for pair in words.windows(2) {
        let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
        match a.iter().zip(b).find(|(x, y)| x != y) {
            Some((&x, &y)) => {
                if adj.get_mut(&x).unwrap().insert(y) {
                    *indeg.get_mut(&y).unwrap() += 1;
                }
            }
            None if a.len() > b.len() => return String::new(),
            None => {}
        }
    }
    let mut q: VecDeque<u8> = indeg.iter().filter(|(_, &d)| d == 0).map(|(&c, _)| c).collect();
    let mut out = String::new();
    while let Some(c) = q.pop_front() {
        out.push(c as char);
        for &nx in &adj[&c] {
            let d = indeg.get_mut(&nx).unwrap();
            *d -= 1;
            if *d == 0 {
                q.push_back(nx);
            }
        }
    }
    if out.len() == indeg.len() { out } else { String::new() }
}

fn main() {
    assert_eq!(alien_order(&["wrt", "wrf", "er", "ett", "rftt"]), "wertf");
    assert_eq!(alien_order(&["z", "x", "z"]), "");
    assert_eq!(alien_order(&["abc", "ab"]), "");
    println!("ok");
}
