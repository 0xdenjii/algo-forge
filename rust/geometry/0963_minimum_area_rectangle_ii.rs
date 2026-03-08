// LeetCode 963 - Minimum Area Rectangle II
// Given distinct points, return the minimum area of any rectangle (not necessarily axis-aligned)
// formed by four of them, or 0 if none.
// Approach: two point pairs are diagonals of a rectangle iff they share midpoint and length;
// group pairs by (sum of coords, squared length) and compare within groups.
// Time: O(n^2 + sum of group^2) | Space: O(n^2)
use std::collections::HashMap;

type P = (i64, i64);

pub fn min_area_free_rect(p: &[P]) -> f64 {
    let mut groups: HashMap<(i64, i64, i64), Vec<(P, P)>> = HashMap::new();
    for i in 0..p.len() {
        for j in i + 1..p.len() {
            let (a, b) = (p[i], p[j]);
            let key = (a.0 + b.0, a.1 + b.1, (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2));
            groups.entry(key).or_default().push((a, b));
        }
    }
    let mut best = f64::INFINITY;
    for v in groups.values() {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (a, b) = v[i];
                let c = v[j].0;
                let side1 = (((a.0 - c.0).pow(2) + (a.1 - c.1).pow(2)) as f64).sqrt();
                let side2 = (((b.0 - c.0).pow(2) + (b.1 - c.1).pow(2)) as f64).sqrt();
                best = best.min(side1 * side2);
            }
        }
    }
    if best.is_finite() { best } else { 0.0 }
}

fn main() {
    assert!((min_area_free_rect(&[(1, 2), (2, 1), (1, 0), (0, 1)]) - 2.0).abs() < 1e-5);
    assert!((min_area_free_rect(&[(0, 1), (2, 1), (1, 1), (1, 0), (2, 0)]) - 1.0).abs() < 1e-5);
    assert_eq!(min_area_free_rect(&[(0, 3), (1, 2), (3, 1), (1, 3), (2, 1)]), 0.0);
    println!("ok");
}
