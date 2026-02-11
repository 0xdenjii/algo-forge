// LeetCode 337 - House Robber III
// Houses form a binary tree; robbing two directly-linked houses alerts police.
// Return the maximum amount that can be robbed.
// Approach: tree DP returning (best if node robbed, best if not robbed).
// Time: O(n), Space: O(h) recursion

pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        Some(Box::new(Node { val, left, right }))
    }
}

fn go(node: &Option<Box<Node>>) -> (i32, i32) {
    match node {
        None => (0, 0),
        Some(n) => {
            let (lr, ls) = go(&n.left);
            let (rr, rs) = go(&n.right);
            (n.val + ls + rs, lr.max(ls) + rr.max(rs))
        }
    }
}

pub fn rob(root: &Option<Box<Node>>) -> i32 {
    let (a, b) = go(root);
    a.max(b)
}

fn main() {
    // [3,2,3,null,3,null,1]
    let t = Node::new(
        3,
        Node::new(2, None, Node::new(3, None, None)),
        Node::new(3, None, Node::new(1, None, None)),
    );
    assert_eq!(rob(&t), 7);
    // [3,4,5,1,3,null,1]
    let t2 = Node::new(
        3,
        Node::new(4, Node::new(1, None, None), Node::new(3, None, None)),
        Node::new(5, None, Node::new(1, None, None)),
    );
    assert_eq!(rob(&t2), 9);
    println!("ok");
}
