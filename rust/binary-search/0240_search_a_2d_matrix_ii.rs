// LeetCode 240 - Search a 2D Matrix II
// Rows are sorted left to right and columns top to bottom (rows need not
// continue each other). Return whether target exists.
// Approach: binary search each row, skipping rows whose range excludes target.
// (The O(m + n) staircase walk is the alternative; this file shows the
// binary-search variant, which wins when n is much larger than m.)
// Time: O(m log n), Space: O(1)

fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    matrix
        .iter()
        .take_while(|row| row.first().is_some_and(|&f| f <= target))
        .any(|row| row.last().is_some_and(|&l| l >= target) && row.binary_search(&target).is_ok())
}

fn main() {
    let m = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert!(search_matrix(&m, 5));
    assert!(!search_matrix(&m, 20));
    assert!(search_matrix(&m, 30));
    println!("ok");
}
