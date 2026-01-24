// LeetCode 715 - Range Module
// Problem: Track half-open real ranges [left, right) over [1, 1e9). Support
//   add_range, remove_range, and query_range, which asks whether every number
//   in [left, right) is currently tracked.
// Approach: A dynamic (implicit) segment tree over the integer cells
//   [1, 1e9 - 1], with a lazy "assign" tag. Nodes are created only when a
//   partial update splits a range. Each node stores whether its whole range is
//   tracked. Index 0 is the "absent" sentinel and the root is index 1.
// Complexity: O(log C) per operation, amortized; space O(Q log C).

const LO: i32 = 1;
const HI: i32 = 1_000_000_000 - 1;

#[derive(Clone, Copy, Default)]
struct Node {
    left: u32,
    right: u32,
    full: bool,
    lazy: Option<bool>,
}

struct RangeModule {
    nodes: Vec<Node>,
}

impl RangeModule {
    fn new() -> Self {
        RangeModule { nodes: vec![Node::default(), Node::default()] }
    }

    fn alloc(&mut self, full: bool) -> u32 {
        // A new child inherits the uniform state of its parent.
        self.nodes.push(Node { full, lazy: Some(full), ..Node::default() });
        (self.nodes.len() - 1) as u32
    }

    fn push_down(&mut self, idx: usize) {
        if self.nodes[idx].left == 0 {
            let f = self.nodes[idx].full;
            let l = self.alloc(f);
            let r = self.alloc(f);
            self.nodes[idx].left = l;
            self.nodes[idx].right = r;
        }
        if let Some(v) = self.nodes[idx].lazy.take() {
            for c in [self.nodes[idx].left, self.nodes[idx].right] {
                let child = &mut self.nodes[c as usize];
                child.full = v;
                child.lazy = Some(v);
            }
        }
    }

    fn assign(&mut self, idx: usize, l: i32, r: i32, ql: i32, qr: i32, v: bool) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.nodes[idx].full = v;
            self.nodes[idx].lazy = Some(v);
            return;
        }
        self.push_down(idx);
        let m = l + (r - l) / 2;
        let (a, b) = (self.nodes[idx].left as usize, self.nodes[idx].right as usize);
        self.assign(a, l, m, ql, qr, v);
        self.assign(b, m + 1, r, ql, qr, v);
        self.nodes[idx].full = self.nodes[a].full && self.nodes[b].full;
    }

    fn covered(&self, idx: usize, l: i32, r: i32, ql: i32, qr: i32) -> bool {
        if qr < l || r < ql {
            return true;
        }
        let node = &self.nodes[idx];
        if node.full {
            return true;
        }
        // Not full: a fully covered, uniform, or leaf-like node means "no".
        if (ql <= l && r <= qr) || node.lazy.is_some() || node.left == 0 {
            return false;
        }
        let m = l + (r - l) / 2;
        self.covered(node.left as usize, l, m, ql, qr)
            && self.covered(node.right as usize, m + 1, r, ql, qr)
    }

    fn add_range(&mut self, left: i32, right: i32) {
        self.assign(1, LO, HI, left, right - 1, true);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.covered(1, LO, HI, left, right - 1)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        self.assign(1, LO, HI, left, right - 1, false);
    }
}

fn main() {
    let mut rm = RangeModule::new();
    rm.add_range(10, 20);
    rm.remove_range(14, 16);
    assert!(rm.query_range(10, 14));
    assert!(!rm.query_range(13, 15));
    assert!(rm.query_range(16, 17));
    rm.add_range(1, 999_999_999);
    assert!(rm.query_range(5, 999_999_999));
    rm.remove_range(500, 501);
    assert!(!rm.query_range(1, 999_999_999));
    assert!(rm.query_range(501, 999_999_999));
    println!("0715 ok");
}
