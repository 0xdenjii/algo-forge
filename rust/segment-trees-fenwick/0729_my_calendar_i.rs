// LeetCode 729 - My Calendar I
// Problem: Implement book(start, end) for half-open intervals [start, end)
//   over times in [0, 1e9]. A booking succeeds only if it would not cause a
//   double booking (two events overlapping).
// Approach: A dynamic segment tree with range add and range max, using
//   permanent lazy tags: tags are never pushed down, and queries add them up
//   along the path. A booking is accepted when the max over [start, end-1]
//   is 0, and then that range is incremented.
// Complexity: O(log C) per booking; space O(B log C) nodes.

#[derive(Clone, Copy, Default)]
struct Node {
    left: u32,
    right: u32,
    max: i32,
    tag: i32,
}

struct DynTree {
    nodes: Vec<Node>, // index 0 = absent sentinel, 1 = root
}

impl DynTree {
    fn new() -> Self {
        DynTree { nodes: vec![Node::default(); 2] }
    }

    fn child(&mut self, idx: usize, right: bool) -> usize {
        let c = if right { self.nodes[idx].right } else { self.nodes[idx].left };
        if c != 0 {
            return c as usize;
        }
        self.nodes.push(Node::default());
        let c = (self.nodes.len() - 1) as u32;
        if right { self.nodes[idx].right = c } else { self.nodes[idx].left = c }
        c as usize
    }

    fn add(&mut self, idx: usize, l: i64, r: i64, ql: i64, qr: i64, v: i32) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.nodes[idx].max += v;
            self.nodes[idx].tag += v;
            return;
        }
        let m = l + (r - l) / 2;
        let a = self.child(idx, false);
        self.add(a, l, m, ql, qr, v);
        let b = self.child(idx, true);
        self.add(b, m + 1, r, ql, qr, v);
        self.nodes[idx].max = self.nodes[a].max.max(self.nodes[b].max) + self.nodes[idx].tag;
    }

    fn max(&self, idx: usize, l: i64, r: i64, ql: i64, qr: i64) -> i32 {
        if qr < l || r < ql {
            return i32::MIN;
        }
        if idx == 0 {
            return 0; // untouched range
        }
        let node = self.nodes[idx];
        if ql <= l && r <= qr {
            return node.max;
        }
        let m = l + (r - l) / 2;
        let best = self
            .max(node.left as usize, l, m, ql, qr)
            .max(self.max(node.right as usize, m + 1, r, ql, qr));
        best + node.tag
    }
}

const HI: i64 = 1_000_000_000;

struct MyCalendar {
    tree: DynTree,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar { tree: DynTree::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let (s, e) = (start as i64, end as i64 - 1);
        if self.tree.max(1, 0, HI, s, e) > 0 {
            return false;
        }
        self.tree.add(1, 0, HI, s, e, 1);
        true
    }
}

fn main() {
    let mut cal = MyCalendar::new();
    assert!(cal.book(10, 20));
    assert!(!cal.book(15, 25));
    assert!(cal.book(20, 30));
    assert!(cal.book(0, 10));
    assert!(!cal.book(5, 6));
    println!("0729 ok");
}
