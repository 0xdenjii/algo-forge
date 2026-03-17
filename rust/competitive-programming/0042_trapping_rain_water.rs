// Problem 42: Trapping Rain Water
// Given n non-negative heights of unit-width bars, compute how much water is trapped after
// raining.
// Approach: Two pointers; the side with the smaller running max bounds the water at that index.
// Complexity: O(n) time, O(1) space.
pub fn trap(h: &[i32]) -> i32 {
    if h.is_empty() {
        return 0;
    }
    let (mut l, mut r) = (0usize, h.len() - 1);
    let (mut lm, mut rm, mut w) = (0, 0, 0);
    while l < r {
        if h[l] < h[r] {
            lm = lm.max(h[l]);
            w += lm - h[l];
            l += 1;
        } else {
            rm = rm.max(h[r]);
            w += rm - h[r];
            r -= 1;
        }
    }
    w
}

fn main() {
    assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    assert_eq!(trap(&[]), 0);
    println!("ok");
}
