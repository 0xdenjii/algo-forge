// LeetCode 173 - Binary Search Tree Iterator
// Implement an iterator over a BST's in-order traversal with next() and has_next(),
// using O(h) memory.
// Approach: controlled in-order traversal with an explicit stack of left spines.
// Rc<RefCell<TreeNode>> matches the LeetCode Rust signature.
// Time: amortized O(1) per next. Space: O(h).
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Tree) -> Self {
        let mut it = Self { stack: Vec::new() };
        it.push_left(root);
        it
    }

    fn push_left(&mut self, mut node: Tree) {
        while let Some(n) = node {
            node = n.borrow().left.clone();
            self.stack.push(n);
        }
    }

    pub fn next(&mut self) -> i32 {
        let n = self.stack.pop().expect("no next element");
        let (val, right) = {
            let b = n.borrow();
            (b.val, b.right.clone())
        };
        self.push_left(right);
        val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn node(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn main() {
    //        7
    //      3   15
    //         9  20
    let root = node(7, node(3, None, None), node(15, node(9, None, None), node(20, None, None)));
    let mut it = BSTIterator::new(root);
    let mut seen = Vec::new();
    while it.has_next() {
        seen.push(it.next());
    }
    assert_eq!(seen, vec![3, 7, 9, 15, 20]);
    println!("bst iterator ok");
}
