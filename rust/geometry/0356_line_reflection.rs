// LeetCode 356 - Line Reflection
// Given points, decide if there is a vertical line that reflects the set onto itself.
// Approach: the only candidate axis is x = (minX + maxX) / 2; check every mirror (using 2*axis to stay integral).
// Time: O(n) | Space: O(n)
use std::collections::HashSet;

pub fn is_reflected(p: &[(i64, i64)]) -> bool {
    if p.is_empty() {
        return true;
    }
    let min = p.iter().map(|q| q.0).min().unwrap();
    let max = p.iter().map(|q| q.0).max().unwrap();
    let s = min + max;
    let set: HashSet<(i64, i64)> = p.iter().copied().collect();
    p.iter().all(|&(x, y)| set.contains(&(s - x, y)))
}

fn main() {
    assert!(is_reflected(&[(1, 1), (-1, 1)]));
    assert!(!is_reflected(&[(1, 1), (-1, -1)]));
    println!("ok");
}
