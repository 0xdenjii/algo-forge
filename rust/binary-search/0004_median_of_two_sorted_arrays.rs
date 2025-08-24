// LeetCode 4 - Median of Two Sorted Arrays
// Given sorted arrays a (len m) and b (len n), return the median of the merged
// array in O(log(min(m, n))).
// Approach: binary search the cut i in the shorter array so that the left
// halves of both arrays together hold (m+n+1)/2 elements and max(left) <= min(right).
// Time: O(log min(m, n)), Space: O(1)

fn find_median_sorted_arrays(a: &[i32], b: &[i32]) -> f64 {
    if a.len() > b.len() {
        return find_median_sorted_arrays(b, a);
    }
    let (m, n) = (a.len(), b.len());
    let half = (m + n + 1) / 2;
    let (mut lo, mut hi) = (0, m);
    loop {
        let i = (lo + hi) / 2;
        let j = half - i;
        let a_left = if i == 0 { i64::MIN } else { a[i - 1] as i64 };
        let a_right = if i == m { i64::MAX } else { a[i] as i64 };
        let b_left = if j == 0 { i64::MIN } else { b[j - 1] as i64 };
        let b_right = if j == n { i64::MAX } else { b[j] as i64 };
        if a_left > b_right {
            hi = i - 1;
        } else if b_left > a_right {
            lo = i + 1;
        } else {
            let left = a_left.max(b_left);
            return if (m + n) % 2 == 1 {
                left as f64
            } else {
                (left + a_right.min(b_right)) as f64 / 2.0
            };
        }
    }
}

fn main() {
    assert_eq!(find_median_sorted_arrays(&[1, 3], &[2]), 2.0);
    assert_eq!(find_median_sorted_arrays(&[1, 2], &[3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(&[], &[1]), 1.0);
    assert_eq!(find_median_sorted_arrays(&[2], &[]), 2.0);
    println!("ok");
}
