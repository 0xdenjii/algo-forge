// LeetCode 208 - Implement Trie (Prefix Tree)
// Support insert(word), search(word) and starts_with(prefix) for lowercase words.
// Approach: arena-allocated nodes with 26 child indices (index 0 = root, so 0 also means "none").
// Time: O(L) per operation, Space: O(nodes * 26)

pub struct Trie {
    children: Vec<[u32; 26]>,
    terminal: Vec<bool>,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie { children: vec![[0; 26]], terminal: vec![false] }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = 0usize;
        for b in word.bytes() {
            let c = (b - b'a') as usize;
            if self.children[node][c] == 0 {
                self.children.push([0; 26]);
                self.terminal.push(false);
                self.children[node][c] = (self.children.len() - 1) as u32;
            }
            node = self.children[node][c] as usize;
        }
        self.terminal[node] = true;
    }

    fn walk(&self, s: &str) -> Option<usize> {
        let mut node = 0usize;
        for b in s.bytes() {
            let next = self.children[node][(b - b'a') as usize];
            if next == 0 {
                return None;
            }
            node = next as usize;
        }
        Some(node)
    }

    pub fn search(&self, word: &str) -> bool {
        self.walk(word).map_or(false, |n| self.terminal[n])
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.walk(prefix).is_some()
    }
}

fn main() {
    let mut t = Trie::new();
    t.insert("apple");
    assert!(t.search("apple"));
    assert!(!t.search("app"));
    assert!(t.starts_with("app"));
    t.insert("app");
    assert!(t.search("app"));
    println!("ok");
}
