// Problem 239: Sliding Window Maximum
// Given an array and a window size k, return the maximum of every contiguous window of size k.
// Approach: Monotonic deque of indices whose values are decreasing; drop expired front index.
// Complexity: O(n) time, O(k) space.
use std::collections::VecDeque;

pub fn max_sliding_window(a: &[i32], k: usize) -> Vec<i32> {
    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut res = Vec::with_capacity(a.len().saturating_sub(k) + 1);
    for i in 0..a.len() {
        while dq.back().is_some_and(|&j| a[j] <= a[i]) {
            dq.pop_back();
        }
        dq.push_back(i);
        if dq[0] + k <= i {
            dq.pop_front();
        }
        if i + 1 >= k {
            res.push(a[dq[0]]);
        }
    }
    res
}

fn main() {
    assert_eq!(max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    assert_eq!(max_sliding_window(&[1], 1), vec![1]);
    println!("ok");
}
