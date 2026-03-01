// LeetCode 223 - Rectangle Area
// Two axis-aligned rectangles are given by bottom-left and top-right corners.
// Return the total area covered by both.
// Approach: sum of areas minus the (clamped) intersection area.
// Time: O(1) | Space: O(1)
#[allow(clippy::too_many_arguments)]
pub fn compute_area(ax1: i64, ay1: i64, ax2: i64, ay2: i64, bx1: i64, by1: i64, bx2: i64, by2: i64) -> i64 {
    let a = (ax2 - ax1) * (ay2 - ay1);
    let b = (bx2 - bx1) * (by2 - by1);
    let w = (ax2.min(bx2) - ax1.max(bx1)).max(0);
    let h = (ay2.min(by2) - ay1.max(by1)).max(0);
    a + b - w * h
}

fn main() {
    assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    println!("ok");
}
