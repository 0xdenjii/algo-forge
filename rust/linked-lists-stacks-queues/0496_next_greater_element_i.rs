// LeetCode 496 - Next Greater Element I
// nums1 is a subset of nums2 (distinct values). For each x in nums1, find the first
// greater element to its right in nums2, or -1.
// Approach: monotonic stack over nums2 builds a value -> next-greater map.
// Time O(n + m), Space O(m).
use std::collections::HashMap;

pub fn next_greater_element(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut next = HashMap::with_capacity(nums2.len());
    let mut st: Vec<i32> = Vec::new();
    for &x in nums2 {
        while st.last().map_or(false, |&t| t < x) { next.insert(st.pop().unwrap(), x); }
        st.push(x);
    }
    nums1.iter().map(|x| *next.get(x).unwrap_or(&-1)).collect()
}

fn main() {
    assert_eq!(next_greater_element(&[4, 1, 2], &[1, 3, 4, 2]), vec![-1, 3, -1]);
    assert_eq!(next_greater_element(&[2, 4], &[1, 2, 3, 4]), vec![3, -1]);
    println!("ok");
}
