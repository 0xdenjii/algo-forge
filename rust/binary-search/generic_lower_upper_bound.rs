// Classic - Generic lower_bound / upper_bound
// Implement C++-style lower_bound (first element >= x) and upper_bound (first
// element > x) generically over any Ord slice, and show the equivalent
// std partition_point / binary_search_by calls.
// Approach: half-open [lo, hi) search on a monotone predicate.
// Time: O(log n), Space: O(1)

/// First index i in [0, n] such that pred(i) is false, given pred is
/// true-then-false over the slice.
fn first_false<T>(s: &[T], pred: impl Fn(&T) -> bool) -> usize {
    let (mut lo, mut hi) = (0, s.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if pred(&s[mid]) { lo = mid + 1 } else { hi = mid }
    }
    lo
}

fn lower_bound<T: Ord>(s: &[T], x: &T) -> usize {
    first_false(s, |v| v < x)
}

fn upper_bound<T: Ord>(s: &[T], x: &T) -> usize {
    first_false(s, |v| v <= x)
}

fn main() {
    let v = [1, 2, 2, 2, 5, 8];
    for x in 0..10 {
        assert_eq!(lower_bound(&v, &x), v.partition_point(|&e| e < x));
        assert_eq!(upper_bound(&v, &x), v.partition_point(|&e| e <= x));
    }
    assert_eq!(lower_bound(&v, &2), 1);
    assert_eq!(upper_bound(&v, &2), 4);
    let words = ["apple", "kiwi", "pear"];
    assert_eq!(lower_bound(&words, &"banana"), 1);
    println!("count of 2 = {}", upper_bound(&v, &2) - lower_bound(&v, &2));
}
