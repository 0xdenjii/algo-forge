// LeetCode 850 - Rectangle Area II
// Given axis-aligned rectangles, return the total area covered by their union modulo 1e9+7.
// Approach: coordinate compression on x and y, mark covered cells in a grid, sum cell areas.
// Time: O(n^3) | Space: O(n^2)  (n <= 200 in the original problem)
pub fn rectangle_area(rects: &[[i64; 4]]) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let mut xs: Vec<i64> = rects.iter().flat_map(|r| [r[0], r[2]]).collect();
    let mut ys: Vec<i64> = rects.iter().flat_map(|r| [r[1], r[3]]).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    let idx = |v: &[i64], x: i64| v.binary_search(&x).unwrap();
    let mut grid = vec![vec![false; ys.len()]; xs.len()];
    for r in rects {
        for row in grid.iter_mut().take(idx(&xs, r[2])).skip(idx(&xs, r[0])) {
            for cell in row.iter_mut().take(idx(&ys, r[3])).skip(idx(&ys, r[1])) {
                *cell = true;
            }
        }
    }
    let mut total = 0i64;
    for i in 0..xs.len().saturating_sub(1) {
        for j in 0..ys.len().saturating_sub(1) {
            if grid[i][j] {
                total = (total + (xs[i + 1] - xs[i]) * (ys[j + 1] - ys[j])) % MOD;
            }
        }
    }
    total
}

fn main() {
    assert_eq!(rectangle_area(&[[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]), 6);
    assert_eq!(rectangle_area(&[[0, 0, 1_000_000_000, 1_000_000_000]]), 49);
    println!("ok");
}
