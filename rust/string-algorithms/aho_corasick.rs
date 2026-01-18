// Classic - Aho-Corasick multi-pattern matching
// Given a dictionary of patterns and a text, report every (position, pattern) occurrence.
// Approach: build a trie, compute failure links by BFS and turn it into a full goto automaton;
// "output links" (dictionary suffix links) enumerate all patterns ending at each position.
// Time: O(sum|P| * SIGMA + |T| + #matches), Space: O(sum|P| * SIGMA)

use std::collections::VecDeque;

const SIGMA: usize = 256;

pub struct AhoCorasick {
    goto: Vec<[u32; SIGMA]>,
    fail: Vec<u32>,
    out: Vec<Option<usize>>,  // pattern ending exactly at this node
    dict: Vec<Option<u32>>,   // nearest proper suffix node that ends a pattern
    lens: Vec<usize>,
}

impl AhoCorasick {
    pub fn new(patterns: &[&str]) -> Self {
        let mut ac = AhoCorasick {
            goto: vec![[u32::MAX; SIGMA]],
            fail: vec![0],
            out: vec![None],
            dict: vec![None],
            lens: patterns.iter().map(|p| p.len()).collect(),
        };
        for (id, p) in patterns.iter().enumerate() {
            let mut v = 0usize;
            for c in p.bytes() {
                if ac.goto[v][c as usize] == u32::MAX {
                    ac.goto.push([u32::MAX; SIGMA]);
                    ac.fail.push(0);
                    ac.out.push(None);
                    ac.dict.push(None);
                    ac.goto[v][c as usize] = (ac.goto.len() - 1) as u32;
                }
                v = ac.goto[v][c as usize] as usize;
            }
            ac.out[v].get_or_insert(id);
        }
        let mut q = VecDeque::new();
        for c in 0..SIGMA {
            match ac.goto[0][c] {
                u32::MAX => ac.goto[0][c] = 0,
                child => q.push_back(child as usize),
            }
        }
        while let Some(v) = q.pop_front() {
            let f = ac.fail[v] as usize;
            ac.dict[v] = if ac.out[f].is_some() { Some(f as u32) } else { ac.dict[f] };
            for c in 0..SIGMA {
                let u = ac.goto[v][c];
                if u == u32::MAX {
                    ac.goto[v][c] = ac.goto[f][c];
                } else {
                    ac.fail[u as usize] = if v == 0 { 0 } else { ac.goto[f][c] };
                    q.push_back(u as usize);
                }
            }
        }
        ac
    }

    /// Returns (start_index, pattern_id) for every occurrence, in order of end position.
    pub fn find_all(&self, text: &str) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut v = 0usize;
        for (i, c) in text.bytes().enumerate() {
            v = self.goto[v][c as usize] as usize;
            let mut node = if self.out[v].is_some() { Some(v as u32) } else { self.dict[v] };
            while let Some(n) = node {
                let id = self.out[n as usize].unwrap();
                res.push((i + 1 - self.lens[id], id));
                node = self.dict[n as usize];
            }
        }
        res
    }
}

fn main() {
    let ac = AhoCorasick::new(&["he", "she", "his", "hers"]);
    let mut hits = ac.find_all("ushers");
    hits.sort();
    assert_eq!(hits, vec![(1, 1), (2, 0), (2, 3)]);
    println!("{hits:?}");
}
