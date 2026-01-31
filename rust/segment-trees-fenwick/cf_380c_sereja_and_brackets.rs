// Codeforces 380C - Sereja and Brackets
// Problem: Given a bracket string s, answer queries (l, r): the length of the
//   longest correct bracket subsequence of s[l..=r].
// Approach: Each segment tree node stores (matched pairs, unmatched '(',
//   unmatched ')'). Merging L and R matches t = min(L.open, R.close) extra
//   pairs. The answer is 2 * matched. The merge is order-sensitive, so the
//   query folds the left and right sides separately.
// Complexity: build O(n), query O(log n); space O(n).

#[derive(Clone, Copy, Default)]
struct Node {
    matched: u32,
    open: u32,
    close: u32,
}

fn merge(a: Node, b: Node) -> Node {
    let t = a.open.min(b.close);
    Node { matched: a.matched + b.matched + t, open: a.open + b.open - t, close: a.close + b.close - t }
}

struct Brackets {
    size: usize,
    t: Vec<Node>,
}

impl Brackets {
    fn new(s: &str) -> Self {
        let size = s.len().next_power_of_two();
        let mut t = vec![Node::default(); 2 * size];
        for (i, c) in s.bytes().enumerate() {
            t[size + i] = if c == b'(' {
                Node { open: 1, ..Node::default() }
            } else {
                Node { close: 1, ..Node::default() }
            };
        }
        for i in (1..size).rev() {
            t[i] = merge(t[2 * i], t[2 * i + 1]);
        }
        Brackets { size, t }
    }

    /// 0-based inclusive range.
    fn query(&self, l: usize, r: usize) -> u32 {
        let (mut l, mut r) = (l + self.size, r + self.size + 1);
        let (mut left, mut right) = (Node::default(), Node::default());
        while l < r {
            if l & 1 == 1 {
                left = merge(left, self.t[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = merge(self.t[r], right);
            }
            l >>= 1;
            r >>= 1;
        }
        2 * merge(left, right).matched
    }
}

fn main() {
    // Official sample; queries are 1-based in the statement.
    let b = Brackets::new("())(())(())(");
    let queries = [(1, 1), (2, 3), (1, 2), (1, 12), (8, 12), (5, 11), (2, 10)];
    let got: Vec<u32> = queries.iter().map(|&(l, r)| b.query(l - 1, r - 1)).collect();
    assert_eq!(got, vec![0, 0, 2, 10, 4, 6, 6]);
    println!("CF 380C ok");
}
