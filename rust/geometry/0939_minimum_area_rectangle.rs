// LeetCode 939 - Minimum Area Rectangle
// Given distinct points, find the minimum area of an axis-aligned rectangle
// whose four corners are all in the set; return 0 if none exists.
// Approach: treat each pair as a diagonal and look up the other two corners in a HashSet.
// Time: O(n^2) | Space: O(n)
use std::collections::HashSet;

pub fn min_area_rect(p: &[(i32, i32)]) -> i64 {
    let set: HashSet<(i32, i32)> = p.iter().copied().collect();
    let mut best = i64::MAX;
    for i in 0..p.len() {
        for j in i + 1..p.len() {
            let (a, b) = (p[i], p[j]);
            if a.0 != b.0 && a.1 != b.1 && set.contains(&(a.0, b.1)) && set.contains(&(b.0, a.1)) {
                best = best.min(((a.0 - b.0) as i64 * (a.1 - b.1) as i64).abs());
            }
        }
    }
    if best == i64::MAX { 0 } else { best }
}

fn main() {
    assert_eq!(min_area_rect(&[(1, 1), (1, 3), (3, 1), (3, 3), (2, 2)]), 4);
    assert_eq!(min_area_rect(&[(1, 1), (1, 3), (3, 1), (3, 3), (4, 1), (4, 3)]), 2);
    println!("ok");
}
