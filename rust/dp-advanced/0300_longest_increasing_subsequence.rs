// LeetCode 300 - Longest Increasing Subsequence
// Return the length of the longest strictly increasing subsequence of nums.
// Approach: patience sorting; tails[k] = smallest tail of an increasing
// subsequence of length k+1, updated via binary search.
// Time: O(n log n), Space: O(n)

pub fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::with_capacity(nums.len());
    for &x in nums {
        match tails.binary_search(&x) {
            Ok(_) => {}
            Err(i) if i == tails.len() => tails.push(x),
            Err(i) => tails[i] = x,
        }
    }
    tails.len()
}

fn main() {
    assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis(&[7, 7, 7, 7]), 1);
    println!("ok");
}
