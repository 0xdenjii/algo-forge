// LeetCode 211 - Design Add and Search Words Data Structure
// add_word(word) stores a word; search(pattern) returns true if any stored word matches,
// where '.' in the pattern matches any single letter.
// Approach: arena-based trie (Vec of nodes, child indices) with DFS branching on '.'.
// Time: add O(L); search O(L) without dots, O(26^d * L) worst case with d dots. Space: O(total chars).
pub struct WordDictionary {
    // nodes[i] = (children indices, 0 = none; is_end)
    nodes: Vec<([u32; 26], bool)>,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self { nodes: vec![([0; 26], false)] }
    }

    pub fn add_word(&mut self, word: &str) {
        let mut cur = 0usize;
        for b in word.bytes() {
            let c = (b - b'a') as usize;
            if self.nodes[cur].0[c] == 0 {
                self.nodes.push(([0; 26], false));
                let id = (self.nodes.len() - 1) as u32;
                self.nodes[cur].0[c] = id;
            }
            cur = self.nodes[cur].0[c] as usize;
        }
        self.nodes[cur].1 = true;
    }

    pub fn search(&self, pattern: &str) -> bool {
        self.dfs(0, pattern.as_bytes())
    }

    fn dfs(&self, node: usize, rest: &[u8]) -> bool {
        match rest.split_first() {
            None => self.nodes[node].1,
            Some((&b'.', tail)) => self.nodes[node]
                .0
                .iter()
                .any(|&ch| ch != 0 && self.dfs(ch as usize, tail)),
            Some((&b, tail)) => {
                let ch = self.nodes[node].0[(b - b'a') as usize];
                ch != 0 && self.dfs(ch as usize, tail)
            }
        }
    }
}

fn main() {
    let mut d = WordDictionary::new();
    d.add_word("bad");
    d.add_word("dad");
    d.add_word("mad");
    assert!(!d.search("pad"));
    assert!(d.search("bad"));
    assert!(d.search(".ad"));
    assert!(d.search("b.."));
    assert!(!d.search("b..."));
    println!("word dictionary ok");
}
