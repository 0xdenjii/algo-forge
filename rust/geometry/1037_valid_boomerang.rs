// LeetCode 1037 - Valid Boomerang
// Three points form a boomerang if they are distinct and not collinear.
// Approach: a non-zero cross product implies both conditions.
// Time: O(1) | Space: O(1)
pub fn is_boomerang(p: [(i32, i32); 3]) -> bool {
    (p[1].0 - p[0].0) * (p[2].1 - p[0].1) != (p[1].1 - p[0].1) * (p[2].0 - p[0].0)
}

fn main() {
    assert!(is_boomerang([(1, 1), (2, 3), (3, 2)]));
    assert!(!is_boomerang([(1, 1), (2, 2), (3, 3)]));
    println!("ok");
}
