// LeetCode 1483 - Kth Ancestor of a Tree Node
// Given parent[] for a tree rooted at 0 (parent[0] = -1), answer queries for the
// k-th ancestor of a node, or -1 if it does not exist.
// Approach: binary lifting table with -1 as the sentinel for "past the root".
// Time: O(n log n) build, O(log k) per query; Space: O(n log n)
pub struct TreeAncestor {
    up: Vec<Vec<i32>>,
}

impl TreeAncestor {
    pub fn new(n: usize, parent: Vec<i32>) -> Self {
        let mut up = vec![parent];
        for k in 1..17 {
            let prev = &up[k - 1];
            let next: Vec<i32> = (0..n).map(|v| if prev[v] < 0 { -1 } else { prev[prev[v] as usize] }).collect();
            up.push(next);
        }
        Self { up }
    }

    pub fn get_kth_ancestor(&self, mut node: i32, k: u32) -> i32 {
        for (bit, row) in self.up.iter().enumerate() {
            if node < 0 {
                break;
            }
            if k >> bit & 1 == 1 {
                node = row[node as usize];
            }
        }
        if (k >> self.up.len()) != 0 { -1 } else { node }
    }
}

fn main() {
    let t = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    assert_eq!(t.get_kth_ancestor(3, 1), 1);
    assert_eq!(t.get_kth_ancestor(5, 2), 0);
    assert_eq!(t.get_kth_ancestor(6, 3), -1);
    println!("ok");
}
