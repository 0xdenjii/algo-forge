// LeetCode 647 - Palindromic Substrings
// Count the palindromic substrings of s (substrings at different positions count separately).
// Approach: expand around each of the 2n-1 centers.
// Time: O(n^2), Space: O(1)

pub fn count_substrings(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len() as isize;
    let mut total = 0;
    for center in 0..(2 * n - 1).max(0) {
        let (mut l, mut r) = (center / 2, center / 2 + center % 2);
        while l >= 0 && r < n && b[l as usize] == b[r as usize] {
            total += 1;
            l -= 1;
            r += 1;
        }
    }
    total
}

fn main() {
    assert_eq!(count_substrings("abc"), 3);
    assert_eq!(count_substrings("aaa"), 6);
    assert_eq!(count_substrings(""), 0);
    println!("ok");
}
