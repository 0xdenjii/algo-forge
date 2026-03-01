// LeetCode 447 - Number of Boomerangs
// Count ordered tuples (i, j, k) where dist(i, j) == dist(i, k).
// Approach: for each pivot i, bucket other points by squared distance; a bucket of size m adds m*(m-1).
// Time: O(n^2) | Space: O(n)
use std::collections::HashMap;

pub fn number_of_boomerangs(p: &[(i64, i64)]) -> usize {
    let mut total = 0;
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &(ax, ay) in p {
        freq.clear();
        for &(bx, by) in p {
            *freq.entry((ax - bx).pow(2) + (ay - by).pow(2)).or_default() += 1;
        }
        total += freq.values().map(|&m| m * (m - 1)).sum::<usize>();
    }
    total
}

fn main() {
    assert_eq!(number_of_boomerangs(&[(0, 0), (1, 0), (2, 0)]), 2);
    assert_eq!(number_of_boomerangs(&[(1, 1), (2, 2), (3, 3)]), 2);
    println!("ok");
}
