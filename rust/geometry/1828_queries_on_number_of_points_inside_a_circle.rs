// LeetCode 1828 - Queries on Number of Points Inside a Circle
// For each query circle (x, y, r), count how many given points lie inside or on it.
// Approach: direct squared-distance check per (query, point) pair; no floating point.
// Time: O(q * n) | Space: O(q)
pub fn count_points(points: &[(i64, i64)], queries: &[(i64, i64, i64)]) -> Vec<usize> {
    queries
        .iter()
        .map(|&(qx, qy, r)| {
            points.iter().filter(|&&(x, y)| (x - qx).pow(2) + (y - qy).pow(2) <= r * r).count()
        })
        .collect()
}

fn main() {
    let pts = [(1, 3), (3, 3), (5, 3), (2, 2)];
    assert_eq!(count_points(&pts, &[(2, 3, 1), (4, 3, 1), (1, 1, 2)]), vec![3, 2, 2]);
    println!("ok");
}
