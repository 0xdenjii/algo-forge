// Classic - Pick's Theorem: Lattice Points of a Polygon
// Given a simple polygon with integer vertices, count lattice points strictly inside (I)
// and on the boundary (B).
// Approach: B = sum of gcd(|dx|,|dy|) over edges; area A by shoelace; Pick: A = I + B/2 - 1,
// so I = (2A - B + 2) / 2.
// Time: O(n log C) | Space: O(1)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

/// Returns (interior, boundary).
pub fn lattice_points(p: &[(i64, i64)]) -> (i64, i64) {
    let n = p.len();
    let (mut twice_area, mut boundary) = (0i64, 0i64);
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        twice_area += a.0 * b.1 - b.0 * a.1;
        boundary += gcd(b.0 - a.0, b.1 - a.1);
    }
    let interior = (twice_area.abs() - boundary + 2) / 2;
    (interior, boundary)
}

fn main() {
    assert_eq!(lattice_points(&[(0, 0), (4, 0), (4, 4), (0, 4)]), (9, 16));
    assert_eq!(lattice_points(&[(0, 0), (4, 0), (0, 4)]), (3, 12));
    println!("ok");
}
