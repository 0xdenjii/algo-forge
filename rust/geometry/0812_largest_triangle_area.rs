// LeetCode 812 - Largest Triangle Area
// Given distinct points, return the area of the largest triangle formed by any three.
// Approach: brute force over triples using the cross-product area formula.
// Time: O(n^3) | Space: O(1)
pub fn largest_triangle_area(p: &[(i64, i64)]) -> f64 {
    let n = p.len();
    let mut best = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let cr = (p[j].0 - p[i].0) * (p[k].1 - p[i].1) - (p[j].1 - p[i].1) * (p[k].0 - p[i].0);
                best = best.max(cr.abs());
            }
        }
    }
    best as f64 / 2.0
}

fn main() {
    let a = largest_triangle_area(&[(0, 0), (0, 1), (1, 0), (0, 2), (2, 0)]);
    assert!((a - 2.0).abs() < 1e-9);
    println!("{a}");
}
