// LeetCode 1283 - Find the Smallest Divisor Given a Threshold
// Choose a positive divisor d, divide every element (rounding up) and sum the
// results. Return the smallest d such that the sum is <= threshold.
// Approach: the sum is non-increasing in d, so binary search d in [1, max].
// Time: O(n log max), Space: O(1)

fn smallest_divisor(nums: &[i32], threshold: i32) -> i32 {
    let total = |d: i32| nums.iter().map(|&x| ((x + d - 1) / d) as i64).sum::<i64>();
    let (mut lo, mut hi) = (1, *nums.iter().max().unwrap());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if total(mid) <= threshold as i64 { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(smallest_divisor(&[1, 2, 5, 9], 6), 5);
    assert_eq!(smallest_divisor(&[44, 22, 33, 11, 1], 5), 44);
    println!("ok");
}
