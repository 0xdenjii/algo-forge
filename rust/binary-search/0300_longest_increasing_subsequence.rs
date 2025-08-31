// LeetCode 300 - Longest Increasing Subsequence
// Return the length of the longest strictly increasing subsequence of nums.
// Approach: patience sorting - tails[i] is the smallest tail of an increasing
// subsequence of length i+1; place each x at lower_bound(tails, x).
// Time: O(n log n), Space: O(n)

fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::with_capacity(nums.len());
    for &x in nums {
        let i = tails.partition_point(|&t| t < x);
        if i == tails.len() {
            tails.push(x);
        } else {
            tails[i] = x;
        }
    }
    tails.len()
}

fn main() {
    assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis(&[7, 7, 7, 7]), 1);
    assert_eq!(length_of_lis(&[]), 0);
    println!("ok");
}
