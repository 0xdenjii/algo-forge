// LeetCode 907 - Sum of Subarray Minimums
// Return the sum of min(b) over every contiguous subarray b, modulo 1e9+7.
// Approach: each arr[i] is the minimum of left[i] * right[i] subarrays, where
// left = distance to previous strictly-smaller element and right = distance to next
// smaller-or-equal element (asymmetric to avoid double counting). Monotonic stacks.
// Time O(n), Space O(n).
pub fn sum_subarray_mins(arr: &[i64]) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let n = arr.len();
    let mut left = vec![0usize; n];
    let mut right = vec![0usize; n];
    let mut st: Vec<usize> = Vec::new();
    for i in 0..n {
        while st.last().map_or(false, |&j| arr[j] >= arr[i]) { st.pop(); }
        left[i] = st.last().map_or(i + 1, |&j| i - j);
        st.push(i);
    }
    st.clear();
    for i in (0..n).rev() {
        while st.last().map_or(false, |&j| arr[j] > arr[i]) { st.pop(); }
        right[i] = st.last().map_or(n - i, |&j| j - i);
        st.push(i);
    }
    (0..n).fold(0, |acc, i| (acc + arr[i] % MOD * (left[i] * right[i]) as i64 % MOD) % MOD)
}

fn main() {
    assert_eq!(sum_subarray_mins(&[3, 1, 2, 4]), 17);
    assert_eq!(sum_subarray_mins(&[11, 81, 94, 43, 3]), 444);
    assert_eq!(sum_subarray_mins(&[2, 2, 2]), 12);
    println!("ok");
}
