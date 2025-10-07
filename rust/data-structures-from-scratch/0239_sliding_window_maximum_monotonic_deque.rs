// LeetCode 239 - Sliding Window Maximum
// Given nums and window size k, return the maximum of each contiguous window of size k.
// Approach: monotonic deque of indices with strictly decreasing values; drop indices that
// fall out of the window from the front and smaller values from the back.
// Time: O(n). Space: O(k).
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut dq: VecDeque<usize> = VecDeque::with_capacity(k);
    let mut out = Vec::with_capacity(nums.len().saturating_sub(k) + 1);
    for (i, &x) in nums.iter().enumerate() {
        if dq.front().map_or(false, |&f| f + k <= i) {
            dq.pop_front();
        }
        while dq.back().map_or(false, |&b| nums[b] <= x) {
            dq.pop_back();
        }
        dq.push_back(i);
        if i + 1 >= k {
            out.push(nums[*dq.front().unwrap()]);
        }
    }
    out
}

fn main() {
    assert_eq!(max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    assert_eq!(max_sliding_window(vec![9, 8, 7, 6], 2), vec![9, 8, 7]);
    println!("sliding window max ok");
}
