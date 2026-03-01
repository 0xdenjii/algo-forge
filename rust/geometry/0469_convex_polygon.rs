// LeetCode 469 - Convex Polygon
// Given polygon vertices in order (simple polygon), return true if it is convex.
// Approach: all non-zero cross products of consecutive edges must share one sign.
// Time: O(n) | Space: O(1)
pub fn is_convex(p: &[(i64, i64)]) -> bool {
    let n = p.len();
    let mut sign = 0i64;
    for i in 0..n {
        let (a, b, c) = (p[i], p[(i + 1) % n], p[(i + 2) % n]);
        let cr = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
        if cr != 0 {
            if sign != 0 && sign != cr.signum() {
                return false;
            }
            sign = cr.signum();
        }
    }
    true
}

fn main() {
    assert!(is_convex(&[(0, 0), (0, 1), (1, 1), (1, 0)]));
    assert!(!is_convex(&[(0, 0), (0, 10), (10, 10), (10, 0), (5, 5)]));
    println!("ok");
}
