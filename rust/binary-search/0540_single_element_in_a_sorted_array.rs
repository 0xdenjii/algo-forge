// LeetCode 540 - Single Element in a Sorted Array
// Every element appears exactly twice except one which appears once. The array
// is sorted. Find the single element in O(log n) time and O(1) space.
// Approach: before the single element, pairs start at even indices; search on
// even mid whether nums[mid] == nums[mid+1].
// Time: O(log n), Space: O(1)

fn single_non_duplicate(nums: &[i32]) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let mut mid = lo + (hi - lo) / 2;
        if mid % 2 == 1 {
            mid -= 1;
        }
        if nums[mid] == nums[mid + 1] { lo = mid + 2 } else { hi = mid }
    }
    nums[lo]
}

fn main() {
    assert_eq!(single_non_duplicate(&[1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(&[3, 3, 7, 7, 10, 11, 11]), 10);
    assert_eq!(single_non_duplicate(&[5]), 5);
    println!("ok");
}
