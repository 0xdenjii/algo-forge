// LeetCode 1539 - Kth Missing Positive Number
// Given a strictly increasing array of positive integers and k, return the
// kth positive integer missing from the array.
// Approach: missing(i) = arr[i] - (i + 1) is non-decreasing; find the first i
// with missing(i) >= k; the answer is i + k.
// Time: O(log n), Space: O(1)

fn find_kth_positive(arr: &[i32], k: i32) -> i32 {
    let (mut lo, mut hi) = (0usize, arr.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] - (mid as i32 + 1) < k { lo = mid + 1 } else { hi = mid }
    }
    lo as i32 + k
}

fn main() {
    assert_eq!(find_kth_positive(&[2, 3, 4, 7, 11], 5), 9);
    assert_eq!(find_kth_positive(&[1, 2, 3, 4], 2), 6);
    assert_eq!(find_kth_positive(&[5], 1), 1);
    println!("ok");
}
