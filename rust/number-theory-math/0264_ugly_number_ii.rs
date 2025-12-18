// LeetCode 264 - Ugly Number II
// Return the n-th ugly number (prime factors only 2, 3, 5; the sequence starts 1, 2, 3, 4, 5, 6, 8...).
// Approach: three pointers merging the streams 2*u, 3*u, 5*u in sorted order, skipping duplicates.
// Time: O(n), Space: O(n)
fn nth_ugly_number(n: usize) -> u64 {
    let mut u = vec![1u64; n];
    let (mut i2, mut i3, mut i5) = (0, 0, 0);
    for k in 1..n {
        let (a, b, c) = (u[i2] * 2, u[i3] * 3, u[i5] * 5);
        u[k] = a.min(b).min(c);
        if u[k] == a { i2 += 1; }
        if u[k] == b { i3 += 1; }
        if u[k] == c { i5 += 1; }
    }
    u[n - 1]
}

fn main() {
    assert_eq!(nth_ugly_number(1), 1);
    assert_eq!(nth_ugly_number(10), 12);
    assert_eq!(nth_ugly_number(1690), 2_123_366_400);
    println!("ok");
}
