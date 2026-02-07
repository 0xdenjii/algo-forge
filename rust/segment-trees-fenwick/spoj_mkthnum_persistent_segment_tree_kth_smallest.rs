// SPOJ MKTHNUM - K-th Number (persistent segment tree)
// Problem: Given an array and many queries (l, r, k), return the k-th smallest
//   value in a[l..=r]. Queries are answered online; the array is static.
// Approach: Compress the values. Version i of a persistent count tree over the
//   value ranks holds the multiset a[0..i]. Each insert copies only the
//   O(log n) nodes on one root-to-leaf path. For a query, version r+1 minus
//   version l gives the counts inside the window; descend left or right by
//   comparing k with the left child's count.
// Complexity: build O(n log n), query O(log n); space O(n log n).

#[derive(Clone, Copy, Default)]
struct Node {
    left: u32,
    right: u32,
    cnt: u32,
}

struct PersistentTree {
    nodes: Vec<Node>, // node 0 is the shared empty tree (children point to itself)
    roots: Vec<u32>,
    sorted: Vec<i64>,
}

impl PersistentTree {
    fn new(a: &[i64]) -> Self {
        let mut sorted = a.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        let mut t = PersistentTree { nodes: vec![Node::default()], roots: vec![0], sorted };
        let m = t.sorted.len();
        for &x in a {
            let rank = t.sorted.binary_search(&x).unwrap();
            let prev = *t.roots.last().unwrap();
            let root = t.insert(prev, 0, m - 1, rank);
            t.roots.push(root);
        }
        t
    }

    fn insert(&mut self, prev: u32, l: usize, r: usize, pos: usize) -> u32 {
        let mut node = self.nodes[prev as usize];
        node.cnt += 1;
        if l < r {
            let m = (l + r) / 2;
            if pos <= m {
                node.left = self.insert(node.left, l, m, pos);
            } else {
                node.right = self.insert(node.right, m + 1, r, pos);
            }
        }
        self.nodes.push(node);
        (self.nodes.len() - 1) as u32
    }

    /// k-th smallest (1-based) in a[l..=r], both 0-based.
    fn kth(&self, l: usize, r: usize, mut k: u32) -> i64 {
        let (mut lo_root, mut hi_root) = (self.roots[l], self.roots[r + 1]);
        let (mut lo, mut hi) = (0, self.sorted.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let a = self.nodes[lo_root as usize];
            let b = self.nodes[hi_root as usize];
            let left_cnt = self.nodes[b.left as usize].cnt - self.nodes[a.left as usize].cnt;
            if k <= left_cnt {
                lo_root = a.left;
                hi_root = b.left;
                hi = mid;
            } else {
                k -= left_cnt;
                lo_root = a.right;
                hi_root = b.right;
                lo = mid + 1;
            }
        }
        self.sorted[lo]
    }
}

fn main() {
    let a = [1i64, 5, 2, 6, 3, 7, 4];
    let t = PersistentTree::new(&a);
    // SPOJ sample (1-based in the statement): (2,5,3)->5, (4,4,1)->6, (1,7,3)->3
    assert_eq!(t.kth(1, 4, 3), 5);
    assert_eq!(t.kth(3, 3, 1), 6);
    assert_eq!(t.kth(0, 6, 3), 3);
    // Exhaustive check against sorting.
    for l in 0..a.len() {
        for r in l..a.len() {
            let mut w = a[l..=r].to_vec();
            w.sort_unstable();
            for k in 1..=w.len() {
                assert_eq!(t.kth(l, r, k as u32), w[k - 1]);
            }
        }
    }
    println!("persistent k-th smallest ok");
}
