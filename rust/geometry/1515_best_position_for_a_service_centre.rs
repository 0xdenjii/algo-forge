// LeetCode 1515 - Best Position for a Service Centre
// Find a point minimizing the sum of Euclidean distances to all customers (geometric median);
// return that minimal sum (answers within 1e-5 accepted).
// Approach: Weiszfeld's iteration starting from the centroid, nudging away from exact vertex hits.
// Time: O(n * iterations) | Space: O(1)
pub fn get_min_dist_sum(p: &[(f64, f64)]) -> f64 {
    let n = p.len() as f64;
    let (mut x, mut y) = (
        p.iter().map(|q| q.0).sum::<f64>() / n,
        p.iter().map(|q| q.1).sum::<f64>() / n,
    );
    let cost = |x: f64, y: f64| p.iter().map(|q| (q.0 - x).hypot(q.1 - y)).sum::<f64>();
    for _ in 0..10_000 {
        let (mut wx, mut wy, mut w) = (0.0, 0.0, 0.0);
        for &(px, py) in p {
            let d = (px - x).hypot(py - y).max(1e-12);
            wx += px / d;
            wy += py / d;
            w += 1.0 / d;
        }
        let (nx, ny) = (wx / w, wy / w);
        let moved = (nx - x).hypot(ny - y);
        x = nx;
        y = ny;
        if moved < 1e-10 {
            break;
        }
    }
    cost(x, y)
}

fn main() {
    let a = get_min_dist_sum(&[(0.0, 1.0), (1.0, 0.0), (1.0, 2.0), (2.0, 1.0)]);
    assert!((a - 4.0).abs() < 1e-5);
    let b = get_min_dist_sum(&[(1.0, 1.0), (3.0, 3.0)]);
    assert!((b - 2.828427).abs() < 1e-5);
    println!("{a:.5} {b:.5}");
}
