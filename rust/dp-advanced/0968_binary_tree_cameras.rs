// LeetCode 968 - Binary Tree Cameras
// A camera on a node monitors its parent, itself and its children. Return the
// minimum number of cameras needed to monitor every node.
// Approach: greedy tree DP with three states per node: NotCovered, Covered,
// HasCamera; place cameras at parents of uncovered leaves (post-order).
// Time: O(n), Space: O(h)

pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    NotCovered,
    Covered,
    Camera,
}

fn dfs(node: &Option<Box<Node>>, cams: &mut u32) -> State {
    let Some(n) = node else { return State::Covered };
    let (l, r) = (dfs(&n.left, cams), dfs(&n.right, cams));
    if l == State::NotCovered || r == State::NotCovered {
        *cams += 1;
        State::Camera
    } else if l == State::Camera || r == State::Camera {
        State::Covered
    } else {
        State::NotCovered
    }
}

pub fn min_camera_cover(root: &Option<Box<Node>>) -> u32 {
    let mut cams = 0;
    if dfs(root, &mut cams) == State::NotCovered {
        cams += 1;
    }
    cams
}

fn leaf() -> Option<Box<Node>> {
    Some(Box::new(Node { left: None, right: None }))
}
fn node(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { left: l, right: r }))
}

fn main() {
    // [0,0,null,0,0]
    assert_eq!(min_camera_cover(&node(node(leaf(), leaf()), None)), 1);
    // chain of 5: [0,0,null,0,null,0,null,null,0]
    let chain = node(node(node(node(leaf(), None), None), None), None);
    assert_eq!(min_camera_cover(&chain), 2);
    assert_eq!(min_camera_cover(&leaf()), 1);
    println!("ok");
}
