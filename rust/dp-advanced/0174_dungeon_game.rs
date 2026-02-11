// LeetCode 174 - Dungeon Game
// A knight starts top-left and must reach bottom-right moving only right/down;
// each cell adds or subtracts health. Find the minimum initial health so that
// health never drops to 0 or below.
// Approach: reverse DP: need[i][j] = max(1, min(need right, need down) - cell).
// Time: O(m*n), Space: O(n)

pub fn calculate_minimum_hp(d: &[Vec<i32>]) -> i32 {
    let (m, n) = (d.len(), d[0].len());
    // need[j] holds the row below; need[n] is a sentinel column.
    let mut need = vec![i32::MAX; n + 1];
    need[n - 1] = 1; // virtual cell below the princess
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let best = need[j].min(need[j + 1]);
            need[j] = (best - d[i][j]).max(1);
        }
    }
    need[0]
}

fn main() {
    let d = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    assert_eq!(calculate_minimum_hp(&d), 7);
    assert_eq!(calculate_minimum_hp(&[vec![0]]), 1);
    assert_eq!(calculate_minimum_hp(&[vec![100]]), 1);
    println!("ok");
}
