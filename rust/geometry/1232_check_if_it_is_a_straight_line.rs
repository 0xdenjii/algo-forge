// LeetCode 1232 - Check If It Is a Straight Line
// Given coordinates, determine whether all points lie on one straight line.
// Approach: cross product of each point relative to the first two must be zero.
// Time: O(n) | Space: O(1)
pub fn check_straight_line(c: &[(i64, i64)]) -> bool {
    let (x0, y0) = c[0];
    let (dx, dy) = (c[1].0 - x0, c[1].1 - y0);
    c.iter().all(|&(x, y)| dx * (y - y0) == dy * (x - x0))
}

fn main() {
    assert!(check_straight_line(&[(1, 2), (2, 3), (3, 4), (4, 5)]));
    assert!(!check_straight_line(&[(1, 1), (2, 2), (3, 4)]));
    println!("ok");
}
