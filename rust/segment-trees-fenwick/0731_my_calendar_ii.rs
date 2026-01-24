// LeetCode 731 - My Calendar II
// Problem: Implement book(start, end) for half-open intervals [start, end).
//   A booking is rejected only if it would cause a triple booking (three
//   events sharing a common moment).
// Approach: A dynamic segment tree with range add and range max, using
//   permanent lazy tags. Accept the booking when the max over [start, end-1]
//   is below 2, then add 1 to that range.
// Complexity: O(log C) per booking; space O(B log C) nodes.

#[derive(Clone, Copy, Default)]
struct Node {
    left: u32,
    right: u32,
    max: i32,
    tag: i32,
}

struct DynTree {
    nodes: Vec<Node>, // 0 = absent sentinel, 1 = root
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
            return 0;
        }
        let node = self.nodes[idx];
        if ql <= l && r <= qr {
            return node.max;
        }
        let m = l + (r - l) / 2;
        self.max(node.left as usize, l, m, ql, qr)
            .max(self.max(node.right as usize, m + 1, r, ql, qr))
            + node.tag
    }
}

const HI: i64 = 1_000_000_000;

struct MyCalendarTwo {
    tree: DynTree,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo { tree: DynTree::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let (s, e) = (start as i64, end as i64 - 1);
        if self.tree.max(1, 0, HI, s, e) >= 2 {
            return false;
        }
        self.tree.add(1, 0, HI, s, e, 1);
        true
    }
}

fn main() {
    let mut cal = MyCalendarTwo::new();
    let expected = [true, true, true, false, true, true];
    let calls = [(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)];
    for (&(s, e), &want) in calls.iter().zip(expected.iter()) {
        assert_eq!(cal.book(s, e), want, "book({s}, {e})");
    }
    println!("0731 ok");
}
