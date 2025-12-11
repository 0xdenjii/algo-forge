// Classic - Binary Search Tree (LeetCode 701 insert / 450 delete / 700 search)
// Implement an unbalanced BST of unique keys with insert, contains, delete (including the
// two-children case) and in-order traversal.
// Approach: Option<Box<Node>> links; delete replaces a two-child node's key with its
// in-order successor (minimum of the right subtree) and removes that successor.
// Time: O(h) per op (O(log n) balanced, O(n) worst). Space: O(n).
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct Bst<T: Ord> {
    root: Link<T>,
    len: usize,
}

impl<T: Ord> Bst<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, key: T) -> bool {
        let mut cur = &mut self.root;
        while let Some(node) = cur {
            cur = match key.cmp(&node.key) {
                std::cmp::Ordering::Less => &mut node.left,
                std::cmp::Ordering::Greater => &mut node.right,
                std::cmp::Ordering::Equal => return false,
            };
        }
        *cur = Some(Box::new(Node { key, left: None, right: None }));
        self.len += 1;
        true
    }

    pub fn contains(&self, key: &T) -> bool {
        let mut cur = &self.root;
        while let Some(node) = cur {
            cur = match key.cmp(&node.key) {
                std::cmp::Ordering::Less => &node.left,
                std::cmp::Ordering::Greater => &node.right,
                std::cmp::Ordering::Equal => return true,
            };
        }
        false
    }

    fn take_min(link: &mut Link<T>) -> T {
        if link.as_ref().unwrap().left.is_some() {
            Self::take_min(&mut link.as_mut().unwrap().left)
        } else {
            let node = link.take().unwrap();
            *link = node.right;
            node.key
        }
    }

    fn remove_at(link: &mut Link<T>, key: &T) -> bool {
        let Some(node) = link else { return false };
        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => Self::remove_at(&mut node.left, key),
            std::cmp::Ordering::Greater => Self::remove_at(&mut node.right, key),
            std::cmp::Ordering::Equal => {
                match (node.left.is_some(), node.right.is_some()) {
                    (true, true) => node.key = Self::take_min(&mut node.right),
                    (true, false) => *link = node.left.take(),
                    _ => *link = node.right.take(),
                }
                true
            }
        }
    }

    pub fn remove(&mut self, key: &T) -> bool {
        let ok = Self::remove_at(&mut self.root, key);
        if ok {
            self.len -= 1;
        }
        ok
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut out = Vec::with_capacity(self.len);
        let mut stack: Vec<&Node<T>> = Vec::new();
        let mut cur = self.root.as_deref();
        while cur.is_some() || !stack.is_empty() {
            while let Some(n) = cur {
                stack.push(n);
                cur = n.left.as_deref();
            }
            let n = stack.pop().unwrap();
            out.push(&n.key);
            cur = n.right.as_deref();
        }
        out
    }
}

fn main() {
    let mut t = Bst::new();
    for k in [50, 30, 70, 20, 40, 60, 80, 35, 45] {
        assert!(t.insert(k));
    }
    assert!(!t.insert(40));
    assert!(t.contains(&45));
    assert!(t.remove(&30)); // two children
    assert!(t.remove(&20)); // leaf
    assert!(t.remove(&70)); // two children
    assert!(!t.remove(&999));
    assert_eq!(t.in_order(), vec![&35, &40, &45, &50, &60, &80]);
    assert_eq!(t.len(), 6);
    println!("bst ok");
}
