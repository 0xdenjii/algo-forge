// LeetCode 208 - Implement Trie (Prefix Tree)
// Support insert(word), search(word) for exact matches, and starts_with(prefix).
// Words consist of lowercase English letters.
// Approach: each node holds 26 optional boxed children and an end-of-word flag.
// Time: O(L) per op for word length L. Space: O(total characters * 26) worst case.
#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    end: bool,
}

#[derive(Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert_with(Box::default);
        }
        node.end = true;
    }

    fn walk(&self, s: &str) -> Option<&Node> {
        let mut node = &self.root;
        for b in s.bytes() {
            node = node.children[(b - b'a') as usize].as_deref()?;
        }
        Some(node)
    }

    pub fn search(&self, word: &str) -> bool {
        self.walk(word).map_or(false, |n| n.end)
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
    println!("trie ok");
}
