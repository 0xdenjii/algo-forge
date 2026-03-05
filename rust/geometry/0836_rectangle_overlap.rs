// LeetCode 836 - Rectangle Overlap
// Rectangles are [x1,y1,x2,y2]. Return true if their intersection has positive area
// (touching edges or corners does not count).
// Approach: projections on both axes must overlap strictly.
// Time: O(1) | Space: O(1)
pub fn is_rectangle_overlap(a: [i32; 4], b: [i32; 4]) -> bool {
    a[0].max(b[0]) < a[2].min(b[2]) && a[1].max(b[1]) < a[3].min(b[3])
}

fn main() {
    assert!(is_rectangle_overlap([0, 0, 2, 2], [1, 1, 3, 3]));
    assert!(!is_rectangle_overlap([0, 0, 1, 1], [1, 0, 2, 1]));
    println!("ok");
}
