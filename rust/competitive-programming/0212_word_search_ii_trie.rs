// Problem 212: Word Search II
// Given a board of lowercase letters and a word list, return every word that can be formed
// from sequentially adjacent (4-directional) cells, using each cell at most once per word.
// Approach: Insert words into a trie; DFS from every cell walking the trie in lockstep, marking
// visited cells in place; take() a found word so it is reported once.
// Complexity: O(RC * 4 * 3^(L-1)) worst-case time, O(total word length) trie space.
#[derive(Default)]
struct Node {
    next: [Option<Box<Node>>; 26],
    word: Option<String>,
}

fn dfs(b: &mut [Vec<u8>], i: usize, j: usize, node: &mut Node, out: &mut Vec<String>) {
    let c = b[i][j];
    if c == b'#' {
        return;
    }
    let Some(nx) = node.next[(c - b'a') as usize].as_deref_mut() else {
        return;
    };
    if let Some(w) = nx.word.take() {
        out.push(w);
    }
    b[i][j] = b'#';
    let (rows, cols) = (b.len(), b[0].len());
    if i > 0 {
        dfs(b, i - 1, j, nx, out);
    }
    if i + 1 < rows {
        dfs(b, i + 1, j, nx, out);
    }
    if j > 0 {
        dfs(b, i, j - 1, nx, out);
    }
    if j + 1 < cols {
        dfs(b, i, j + 1, nx, out);
    }
    b[i][j] = c;
}

pub fn find_words(board: &mut [Vec<u8>], words: &[&str]) -> Vec<String> {
    let mut root = Node::default();
    for w in words {
        let mut cur = &mut root;
        for c in w.bytes() {
            cur = cur.next[(c - b'a') as usize].get_or_insert_with(Default::default);
        }
        cur.word = Some(w.to_string());
    }
    let mut out = Vec::new();
    if board.is_empty() {
        return out;
    }
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            dfs(board, i, j, &mut root, &mut out);
        }
    }
    out
}

fn main() {
    let mut b: Vec<Vec<u8>> =
        ["oaan", "etae", "ihkr", "iflv"].iter().map(|s| s.bytes().collect()).collect();
    let mut r = find_words(&mut b, &["oath", "pea", "eat", "rain"]);
    r.sort();
    assert_eq!(r, vec!["eat", "oath"]);
    println!("ok");
}
