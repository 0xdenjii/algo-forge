// LeetCode 239 - Sliding Window Maximum
// Return the maximum of each window of size k sliding over the array.
// Approach: monotonic deque of indices with decreasing values; front is the max,
// expired indices are dropped from the front.
// Time O(n), Space O(k).
use std::collections::VecDeque;

pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut out = Vec::with_capacity(nums.len().saturating_sub(k) + 1);
    for (i, &x) in nums.iter().enumerate() {
        if dq.front().map_or(false, |&f| f + k <= i) { dq.pop_front(); }
        while dq.back().map_or(false, |&b| nums[b] <= x) { dq.pop_back(); }
        dq.push_back(i);
        if i + 1 >= k { out.push(nums[*dq.front().unwrap()]); }
    }
    out
}

fn main() {
    assert_eq!(max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    assert_eq!(max_sliding_window(&[1], 1), vec![1]);
    println!("ok");
}
