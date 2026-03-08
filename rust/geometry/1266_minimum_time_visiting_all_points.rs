// LeetCode 1266 - Minimum Time Visiting All Points
// Visit points in order; each second you may move one unit horizontally, vertically or diagonally.
// Return the minimum total time.
// Approach: time between two points is their Chebyshev distance max(|dx|, |dy|).
// Time: O(n) | Space: O(1)
pub fn min_time_to_visit_all_points(p: &[(i32, i32)]) -> i32 {
    p.windows(2).map(|w| (w[0].0 - w[1].0).abs().max((w[0].1 - w[1].1).abs())).sum()
}

fn main() {
    assert_eq!(min_time_to_visit_all_points(&[(1, 1), (3, 4), (-1, 0)]), 7);
    assert_eq!(min_time_to_visit_all_points(&[(3, 2), (-2, 2)]), 5);
    println!("ok");
}
