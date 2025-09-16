// LeetCode 367 - Valid Perfect Square
// Given a positive integer num, return true if it is a perfect square without
// using any built-in sqrt.
// Approach: binary search m in [1, num] comparing m*m to num in i64.
// Time: O(log n), Space: O(1)

use std::cmp::Ordering;

fn is_perfect_square(num: i32) -> bool {
    let n = num as i64;
    let (mut lo, mut hi) = (1i64, n);
    while lo <= hi {
        let m = lo + (hi - lo) / 2;
        match (m * m).cmp(&n) {
            Ordering::Equal => return true,
            Ordering::Less => lo = m + 1,
            Ordering::Greater => hi = m - 1,
        }
    }
    false
}

fn main() {
    assert!(is_perfect_square(16));
    assert!(!is_perfect_square(14));
    assert!(is_perfect_square(1));
    assert!(!is_perfect_square(i32::MAX));
    println!("ok");
}
