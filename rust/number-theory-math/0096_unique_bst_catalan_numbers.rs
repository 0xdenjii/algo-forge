// LeetCode 96 - Unique Binary Search Trees (Catalan numbers)
// Return the number of structurally unique BSTs storing values 1..=n.
// Approach: the answer is the n-th Catalan number C(n) = C(2n, n) / (n + 1); computed
// incrementally via C(k+1) = C(k) * 2(2k+1) / (k+2), which stays exact in integers.
// Time: O(n), Space: O(1)
fn num_trees(n: u32) -> u64 {
    let mut c: u64 = 1;
    for k in 0..n as u64 {
        c = c * 2 * (2 * k + 1) / (k + 2);
    }
    c
}

/// Reference O(n^2) DP: G(n) = sum_{i=1..n} G(i-1) * G(n-i).
fn num_trees_dp(n: usize) -> u64 {
    let mut g = vec![0u64; n + 1];
    g[0] = 1;
    for m in 1..=n {
        g[m] = (1..=m).map(|i| g[i - 1] * g[m - i]).sum();
    }
    g[n]
}

fn main() {
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(1), 1);
    assert!((0..=19).all(|n| num_trees(n) == num_trees_dp(n as usize)));
    println!("C(19) = {}", num_trees(19));
}
