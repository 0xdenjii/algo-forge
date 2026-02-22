// LeetCode 310 - Minimum Height Trees
// For a tree of n nodes, return all roots that minimize the tree height.
// Approach: topological "leaf peeling": repeatedly strip the current leaves; the last
// one or two remaining nodes are the centers of the tree.
// Time: O(n), Space: O(n)
pub fn find_min_height_trees(n: usize, edges: &[[usize; 2]]) -> Vec<usize> {
    if n <= 2 {
        return (0..n).collect();
    }
    let mut adj = vec![Vec::new(); n];
    let mut deg = vec![0; n];
    for &[a, b] in edges {
        adj[a].push(b);
        adj[b].push(a);
        deg[a] += 1;
        deg[b] += 1;
    }
    let mut leaves: Vec<usize> = (0..n).filter(|&i| deg[i] == 1).collect();
    let mut remaining = n;
    while remaining > 2 {
        remaining -= leaves.len();
        let mut next = Vec::new();
        for &leaf in &leaves {
            for &nb in &adj[leaf] {
                deg[nb] -= 1;
                if deg[nb] == 1 {
                    next.push(nb);
                }
            }
        }
        leaves = next;
    }
    leaves.sort_unstable();
    leaves
}

fn main() {
    assert_eq!(find_min_height_trees(4, &[[1, 0], [1, 2], [1, 3]]), vec![1]);
    assert_eq!(find_min_height_trees(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]), vec![3, 4]);
    println!("ok");
}
