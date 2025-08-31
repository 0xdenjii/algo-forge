// LeetCode 278 - First Bad Version
// Versions are 1..=n; once a version is bad every later one is bad. Given an
// API is_bad(v), find the first bad version with the fewest API calls.
// Approach: binary search on the monotone predicate.
// Time: O(log n) calls, Space: O(1)

fn first_bad_version(n: u32, is_bad: impl Fn(u32) -> bool) -> u32 {
    let (mut lo, mut hi) = (1u32, n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if is_bad(mid) {
            hi = mid
        } else {
            lo = mid + 1
        }
    }
    lo
}

fn main() {
    assert_eq!(first_bad_version(5, |v| v >= 4), 4);
    assert_eq!(first_bad_version(1, |v| v >= 1), 1);
    assert_eq!(first_bad_version(u32::MAX, |v| v >= 1_702_766_719), 1_702_766_719);
    println!("ok");
}
