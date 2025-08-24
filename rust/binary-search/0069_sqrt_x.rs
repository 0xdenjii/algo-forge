// LeetCode 69 - Sqrt(x)
// Given a non-negative integer x, return floor(sqrt(x)) without using any
// built-in exponent or sqrt function.
// Approach: binary search the largest m with m*m <= x, using u64 arithmetic.
// Time: O(log x), Space: O(1)

fn my_sqrt(x: i32) -> i32 {
    let x = x as u64;
    // invariant: lo*lo <= x < hi*hi
    let (mut lo, mut hi) = (0u64, x.min(1 << 16) + 1);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if mid * mid <= x {
            lo = mid
        } else {
            hi = mid
        }
    }
    lo as i32
}

fn main() {
    assert_eq!(my_sqrt(0), 0);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(i32::MAX), 46340);
    println!("ok");
}
