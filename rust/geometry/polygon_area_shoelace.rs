// Classic - Polygon Area (Shoelace Formula)
// Given the vertices of a simple polygon in order (CW or CCW), compute its area
// and its orientation.
// Approach: signed area = 1/2 * sum(x_i*y_{i+1} - x_{i+1}*y_i); sign gives orientation.
// Time: O(n) | Space: O(1)
/// Returns twice the signed area (exact for integer coordinates). Positive means counter-clockwise.
pub fn twice_signed_area(p: &[(i64, i64)]) -> i64 {
    let n = p.len();
    (0..n).map(|i| {
        let (a, b) = (p[i], p[(i + 1) % n]);
        a.0 * b.1 - b.0 * a.1
    }).sum()
}

pub fn area(p: &[(i64, i64)]) -> f64 {
    twice_signed_area(p).abs() as f64 / 2.0
}

pub fn is_ccw(p: &[(i64, i64)]) -> bool {
    twice_signed_area(p) > 0
}

fn main() {
    let sq = [(0, 0), (4, 0), (4, 4), (0, 4)];
    assert_eq!(area(&sq), 16.0);
    assert!(is_ccw(&sq));
    let l_shape = [(0, 0), (0, 3), (1, 3), (1, 1), (3, 1), (3, 0)];
    assert_eq!(area(&l_shape), 5.0);
    assert!(!is_ccw(&l_shape));
    println!("ok");
}
