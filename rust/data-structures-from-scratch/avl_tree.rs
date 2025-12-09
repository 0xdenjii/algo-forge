// Classic - AVL Tree (self-balancing BST)
// Ordered set with insert, remove, contains, and k-th smallest, guaranteeing O(log n) height.
// Approach: each node stores height and subtree size; after every insert/remove on the way back
// up, recompute and apply single or double rotations when |balance| > 1.
// Time: O(log n) per op. Space: O(n).
use std::cmp::Ordering;

type Link = Option<Box<Node>>;

struct Node {
    key: i64,
    height: i32,
    size: usize,
    left: Link,
    right: Link,
}

fn h(l: &Link) -> i32 {
    l.as_ref().map_or(0, |n| n.height)
}
fn sz(l: &Link) -> usize {
    l.as_ref().map_or(0, |n| n.size)
}

impl Node {
    fn new(key: i64) -> Box<Node> {
        Box::new(Node { key, height: 1, size: 1, left: None, right: None })
    }
    fn fix(&mut self) {
        self.height = 1 + h(&self.left).max(h(&self.right));
        self.size = 1 + sz(&self.left) + sz(&self.right);
    }
    fn balance(&self) -> i32 {
        h(&self.left) - h(&self.right)
    }
}

fn rotate_right(mut n: Box<Node>) -> Box<Node> {
    let mut l = n.left.take().unwrap();
    n.left = l.right.take();
    n.fix();
    l.right = Some(n);
    l.fix();
    l
}

fn rotate_left(mut n: Box<Node>) -> Box<Node> {
    let mut r = n.right.take().unwrap();
    n.right = r.left.take();
    n.fix();
    r.left = Some(n);
    r.fix();
    r
}

fn rebalance(mut n: Box<Node>) -> Box<Node> {
    n.fix();
    let b = n.balance();
    if b > 1 {
        if n.left.as_ref().unwrap().balance() < 0 {
            n.left = Some(rotate_left(n.left.take().unwrap()));
        }
        return rotate_right(n);
    }
    if b < -1 {
        if n.right.as_ref().unwrap().balance() > 0 {
            n.right = Some(rotate_right(n.right.take().unwrap()));
        }
        return rotate_left(n);
    }
    n
}

fn insert(link: Link, key: i64, added: &mut bool) -> Box<Node> {
    let Some(mut n) = link else {
        *added = true;
        return Node::new(key);
    };
    match key.cmp(&n.key) {
        Ordering::Less => n.left = Some(insert(n.left.take(), key, added)),
        Ordering::Greater => n.right = Some(insert(n.right.take(), key, added)),
        Ordering::Equal => return n,
    }
    rebalance(n)
}

fn pop_min(mut n: Box<Node>) -> (Link, i64) {
    match n.left.take() {
        None => (n.right.take(), n.key),
        Some(l) => {
            let (nl, k) = pop_min(l);
            n.left = nl;
            (Some(rebalance(n)), k)
        }
    }
}

fn remove(link: Link, key: i64, removed: &mut bool) -> Link {
    let mut n = link?;
    match key.cmp(&n.key) {
        Ordering::Less => n.left = remove(n.left.take(), key, removed),
        Ordering::Greater => n.right = remove(n.right.take(), key, removed),
        Ordering::Equal => {
            *removed = true;
            match (n.left.take(), n.right.take()) {
                (None, r) => return r,
                (l, None) => return l,
                (l, Some(r)) => {
                    let (nr, succ) = pop_min(r);
                    n.key = succ;
                    n.left = l;
                    n.right = nr;
                }
            }
        }
    }
    Some(rebalance(n))
}

#[derive(Default)]
pub struct AvlSet {
    root: Link,
}

impl AvlSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        sz(&self.root)
    }
    pub fn height(&self) -> i32 {
        h(&self.root)
    }
    pub fn insert(&mut self, key: i64) -> bool {
        let mut added = false;
        self.root = Some(insert(self.root.take(), key, &mut added));
        added
    }
    pub fn remove(&mut self, key: i64) -> bool {
        let mut removed = false;
        self.root = remove(self.root.take(), key, &mut removed);
        removed
    }
    pub fn contains(&self, key: i64) -> bool {
        let mut cur = &self.root;
        while let Some(n) = cur {
            match key.cmp(&n.key) {
                Ordering::Less => cur = &n.left,
                Ordering::Greater => cur = &n.right,
                Ordering::Equal => return true,
            }
        }
        false
    }
    /// 0-indexed k-th smallest key.
    pub fn kth(&self, mut k: usize) -> Option<i64> {
        let mut cur = &self.root;
        while let Some(n) = cur {
            let ls = sz(&n.left);
            match k.cmp(&ls) {
                Ordering::Less => cur = &n.left,
                Ordering::Equal => return Some(n.key),
                Ordering::Greater => {
                    k -= ls + 1;
                    cur = &n.right;
                }
            }
        }
        None
    }
}

fn main() {
    let mut s = AvlSet::new();
    for i in 0..10_000 {
        s.insert(i); // sorted inserts would degrade a plain BST to a list
    }
    assert_eq!(s.len(), 10_000);
    assert!(s.height() <= 20, "height {}", s.height());
    for i in (0..10_000).step_by(2) {
        assert!(s.remove(i));
    }
    assert!(!s.remove(0));
    assert_eq!(s.len(), 5_000);
    assert!(s.contains(9_999) && !s.contains(9_998));
    assert_eq!(s.kth(0), Some(1));
    assert_eq!(s.kth(10), Some(21));
    assert_eq!(s.kth(5_000), None);
    println!("avl ok, height = {}", s.height());
}
