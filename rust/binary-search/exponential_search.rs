// Classic - Exponential (Galloping) Search
// Search a sorted sequence of unknown or huge length (like LeetCode 702,
// "Search in a Sorted Array of Unknown Size") given only get(i) -> Option.
// Approach: double the bound until it passes target or the end, then binary
// search inside [bound/2, bound].
// Time: O(log p) where p is the target position, Space: O(1)

fn exponential_search(get: impl Fn(usize) -> Option<i32>, target: i32) -> Option<usize> {
    if get(0)? == target {
        return Some(0);
    }
    let mut bound = 1usize;
    while matches!(get(bound), Some(v) if v < target) {
        bound *= 2;
    }
    // answer, if any, lies in (bound/2, bound]; None reads count as +infinity
    let (mut lo, mut hi) = (bound / 2 + 1, bound + 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match get(mid) {
            Some(v) if v < target => lo = mid + 1,
            _ => hi = mid,
        }
    }
    (get(lo) == Some(target)).then_some(lo)
}

fn main() {
    let data: Vec<i32> = (0..1000).map(|i| i * 3).collect();
    let get = |i: usize| data.get(i).copied();
    assert_eq!(exponential_search(get, 0), Some(0));
    assert_eq!(exponential_search(get, 300), Some(100));
    assert_eq!(exponential_search(get, 2997), Some(999));
    assert_eq!(exponential_search(get, 301), None);
    assert_eq!(exponential_search(get, 5000), None);
    println!("ok");
}
