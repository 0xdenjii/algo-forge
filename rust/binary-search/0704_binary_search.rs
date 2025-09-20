// LeetCode 704 - Binary Search
// Given a sorted array of distinct integers and a target, return the index of
// target or -1 if it is absent.
// Approach: classic half-open [lo, hi) binary search.
// Time: O(log n), Space: O(1)

use std::cmp::Ordering;

fn search(nums: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0usize, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }
    -1
}

fn main() {
    assert_eq!(search(&[-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(&[-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(search(&[], 1), -1);
    println!("ok");
}
