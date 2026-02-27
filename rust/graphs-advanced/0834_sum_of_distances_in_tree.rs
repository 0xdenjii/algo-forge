// LeetCode 834 - Sum of Distances in Tree
// For every node of an n-node tree, return the sum of distances to all other nodes.
// Approach: rerooting DP. Pass 1 (post-order) computes subtree sizes and the answer for
// root 0. Pass 2 (pre-order): moving root from p to child c gives
// ans[c] = ans[p] - size[c] + (n - size[c]).
// Time: O(n), Space: O(n) (iterative, no recursion)
pub fn sum_of_distances_in_tree(n: usize, edges: &[[usize; 2]]) -> Vec<i64> {
    let mut adj = vec![Vec::new(); n];
    for &[a, b] in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut st = vec![0];
    parent[0] = 0;
    while let Some(u) = st.pop() {
        order.push(u);
        for &v in &adj[u] {
            if parent[v] == usize::MAX {
                parent[v] = u;
                st.push(v);
            }
        }
    }
    let mut size = vec![1i64; n];
    let mut down = vec![0i64; n]; // sum of distances within subtree
    for &u in order.iter().rev().filter(|&&u| u != 0) {
        let p = parent[u];
        size[p] += size[u];
        down[p] += down[u] + size[u];
    }
    let mut ans = vec![0i64; n];
    ans[0] = down[0];
    for &u in order.iter().filter(|&&u| u != 0) {
        ans[u] = ans[parent[u]] - size[u] + (n as i64 - size[u]);
    }
    ans
}

fn main() {
    let e = [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]];
    assert_eq!(sum_of_distances_in_tree(6, &e), vec![8, 12, 6, 10, 10, 10]);
    assert_eq!(sum_of_distances_in_tree(1, &[]), vec![0]);
    println!("ok");
}
