// LeetCode 503 - Next Greater Element II
// In a circular array, find for each element the next greater value, or -1.
// Approach: iterate indices 0..2n (mod n) with a monotonic stack of indices;
// only push during the first pass.
// Time O(n), Space O(n).
pub fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![-1; n];
    let mut st: Vec<usize> = Vec::new();
    for k in 0..2 * n {
        let x = nums[k % n];
        while st.last().map_or(false, |&j| nums[j] < x) { ans[st.pop().unwrap()] = x; }
        if k < n { st.push(k); }
    }
    ans
}

fn main() {
    assert_eq!(next_greater_elements(&[1, 2, 1]), vec![2, -1, 2]);
    assert_eq!(next_greater_elements(&[1, 2, 3, 4, 3]), vec![2, 3, 4, -1, 4]);
    println!("ok");
}
