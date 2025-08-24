// LeetCode 35 - Search Insert Position
// Given a sorted array of distinct integers and a target, return its index if
// found, otherwise the index where it would be inserted to keep order.
// Approach: lower_bound - first index with nums[i] >= target (partition_point).
// Time: O(log n), Space: O(1)

fn search_insert(nums: &[i32], target: i32) -> usize {
    nums.partition_point(|&x| x < target)
}

fn main() {
    assert_eq!(search_insert(&[1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(&[1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(&[1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(&[1, 3, 5, 6], 0), 0);
    println!("ok");
}
