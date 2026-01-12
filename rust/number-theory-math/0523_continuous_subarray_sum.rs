// LeetCode 523 - Continuous Subarray Sum
// Given non-negative nums and k >= 1, is there a subarray of length >= 2 whose sum is a multiple of k?
// Approach: prefix sums modulo k; two equal residues at indices at least 2 apart give a valid subarray.
// Store only the first index of each residue (seed residue 0 at index -1).
// Time: O(n), Space: O(min(n, k))
use std::collections::HashMap;

fn check_subarray_sum(nums: &[i32], k: i32) -> bool {
    let mut first: HashMap<i32, i64> = HashMap::from([(0, -1)]);
    let mut s = 0;
    for (i, &x) in nums.iter().enumerate() {
        s = (s + x) % k;
        match first.get(&s) {
            Some(&j) if i as i64 - j >= 2 => return true,
            Some(_) => {}
            None => {
                first.insert(s, i as i64);
            }
        }
    }
    false
}

fn main() {
    assert!(check_subarray_sum(&[23, 2, 4, 6, 7], 6));
    assert!(check_subarray_sum(&[23, 2, 6, 4, 7], 6));
    assert!(!check_subarray_sum(&[23, 2, 6, 4, 7], 13));
    println!("ok");
}
