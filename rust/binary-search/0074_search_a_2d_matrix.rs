// LeetCode 74 - Search a 2D Matrix
// Each row is sorted and the first element of each row is greater than the
// last element of the previous row. Return whether target is in the matrix.
// Approach: treat the m x n matrix as a flat sorted array of length m*n.
// Time: O(log(m*n)), Space: O(1)

use std::cmp::Ordering;

fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    let (m, n) = (matrix.len(), matrix.first().map_or(0, Vec::len));
    let (mut lo, mut hi) = (0, m * n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match matrix[mid / n][mid % n].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }
    false
}

fn main() {
    let m = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(search_matrix(&m, 3));
    assert!(!search_matrix(&m, 13));
    assert!(!search_matrix(&[], 1));
    println!("ok");
}
