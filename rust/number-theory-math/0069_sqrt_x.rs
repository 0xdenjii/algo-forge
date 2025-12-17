// LeetCode 69 - Sqrt(x)
// Return floor(sqrt(x)) for a non-negative integer x without using a built-in sqrt.
// Approach: binary search for the largest m with m*m <= x (u64 avoids overflow).
// Time: O(log x), Space: O(1)
fn my_sqrt(x: i32) -> i32 {
    let x = x as u64;
    let (mut lo, mut hi) = (0u64, x.min(1 << 16) + 1); // invariant: lo*lo <= x < hi*hi
    while lo + 1 < hi {
        let m = (lo + hi) / 2;
        if m * m <= x { lo = m } else { hi = m }
    }
    lo as i32
}

fn main() {
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(i32::MAX), 46340);
    println!("ok");
}
