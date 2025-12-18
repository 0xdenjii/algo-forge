// LeetCode 263 - Ugly Number
// An ugly number is a positive integer whose only prime factors are 2, 3 and 5. Test n.
// Approach: divide out 2, 3 and 5 as long as possible and check the remainder is 1.
// Time: O(log n), Space: O(1)
fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    for p in [2, 3, 5] {
        while n % p == 0 {
            n /= p;
        }
    }
    n == 1
}

fn main() {
    assert!(is_ugly(6));
    assert!(is_ugly(1));
    assert!(!is_ugly(14));
    assert!(!is_ugly(0));
    println!("ok");
}
