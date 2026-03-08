// LeetCode 1401 - Circle and Rectangle Overlapping
// Given a circle (radius, center) and an axis-aligned rectangle (x1,y1,x2,y2),
// return true if they share at least one point.
// Approach: clamp the center onto the rectangle to get the nearest point; compare distance to radius.
// Time: O(1) | Space: O(1)
pub fn check_overlap(r: i64, xc: i64, yc: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let dx = xc.clamp(x1, x2) - xc;
    let dy = yc.clamp(y1, y2) - yc;
    dx * dx + dy * dy <= r * r
}

fn main() {
    assert!(check_overlap(1, 0, 0, 1, -1, 3, 1));
    assert!(!check_overlap(1, 1, 1, 1, -3, 2, -1));
    println!("ok");
}
