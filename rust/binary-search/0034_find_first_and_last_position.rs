// LeetCode 34 - Find First and Last Position of Element in Sorted Array
// Given a non-decreasing array and a target, return [first, last] index of
// target, or [-1, -1] if absent. Must run in O(log n).
// Approach: two partition points: first x >= target and first x > target.
// Time: O(log n), Space: O(1)

fn search_range(nums: &[i32], target: i32) -> [i32; 2] {
    let l = nums.partition_point(|&x| x < target);
    let r = nums.partition_point(|&x| x <= target);
    if l == r {
        [-1, -1]
    } else {
        [l as i32, r as i32 - 1]
    }
}

fn main() {
    assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 8), [3, 4]);
    assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 6), [-1, -1]);
    assert_eq!(search_range(&[], 0), [-1, -1]);
    println!("ok");
}
