// LeetCode 154 - Find Minimum in Rotated Sorted Array II
// Same as #153 but the array may contain duplicates.
// Approach: as #153; when nums[mid] == nums[hi], hi can be dropped safely
// because nums[mid] still represents that value.
// Time: O(log n) average, O(n) worst case, Space: O(1)

use std::cmp::Ordering;

fn find_min(nums: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match nums[mid].cmp(&nums[hi]) {
            Ordering::Greater => lo = mid + 1,
            Ordering::Less => hi = mid,
            Ordering::Equal => hi -= 1,
        }
    }
    nums[lo]
}

fn main() {
    assert_eq!(find_min(&[1, 3, 5]), 1);
    assert_eq!(find_min(&[2, 2, 2, 0, 1]), 0);
    assert_eq!(find_min(&[10, 1, 10, 10, 10]), 1);
    println!("ok");
}
