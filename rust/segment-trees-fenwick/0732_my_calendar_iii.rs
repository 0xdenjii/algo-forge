// LeetCode 732 - My Calendar III
// Problem: Each book(start, end) adds the event [start, end) and returns the
//   maximum k such that some moment is covered by k events (the largest
//   k-booking so far).
// Approach: A dynamic segment tree with range add and global max. After
//   adding 1 on [start, end-1], the root's max is the answer. Permanent tags
//   mean nothing is ever pushed down.
// Complexity: O(log C) per booking; space O(B log C) nodes.

#[derive(Clone, Copy, Default)]
struct Node {
    left: u32,
    right: u32,
    max: i32,
    tag: i32,
}

struct MyCalendarThree {
    nodes: Vec<Node>, // 0 = absent sentinel, 1 = root
}

const HI: i64 = 1_000_000_000;

impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree { nodes: vec![Node::default(); 2] }
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

    fn add(&mut self, idx: usize, l: i64, r: i64, ql: i64, qr: i64) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.nodes[idx].max += 1;
            self.nodes[idx].tag += 1;
            return;
        }
        let m = l + (r - l) / 2;
        let a = self.child(idx, false);
        self.add(a, l, m, ql, qr);
        let b = self.child(idx, true);
        self.add(b, m + 1, r, ql, qr);
        self.nodes[idx].max = self.nodes[a].max.max(self.nodes[b].max) + self.nodes[idx].tag;
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.add(1, 0, HI, start as i64, end as i64 - 1);
        self.nodes[1].max
    }
}

fn main() {
    let mut cal = MyCalendarThree::new();
    let calls = [(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)];
    let expected = [1, 1, 2, 3, 3, 3];
    for (&(s, e), &want) in calls.iter().zip(expected.iter()) {
        assert_eq!(cal.book(s, e), want);
    }
    println!("0732 ok");
}
