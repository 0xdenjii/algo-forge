// LeetCode 153 - Find Minimum in Rotated Sorted Array
// Given a rotated sorted array of unique elements, return the minimum element
// in O(log n).
// Approach: compare mid with the right end; the minimum lies in the unsorted half.
// Time: O(log n), Space: O(1)

fn find_min(nums: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[hi] {
            lo = mid + 1
        } else {
            hi = mid
        }
    }
    nums[lo]
}

fn main() {
    assert_eq!(find_min(&[3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(&[4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(&[11, 13, 15, 17]), 11);
    println!("ok");
}
