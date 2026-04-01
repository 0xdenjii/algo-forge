// Problem 300: Longest Increasing Subsequence
// Return the length of the longest strictly increasing subsequence of an integer array.
// Approach: Patience sorting: tails[k] = smallest tail of an increasing subsequence of length
// k + 1; replace via binary search (partition_point).
// Complexity: O(n log n) time, O(n) space.
pub fn length_of_lis(a: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in a {
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
    println!("ok");
}
