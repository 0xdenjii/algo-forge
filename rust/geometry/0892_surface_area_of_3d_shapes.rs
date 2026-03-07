// LeetCode 892 - Surface Area of 3D Shapes
// grid[i][j] cubes are stacked on cell (i, j). Return the total surface area of the shapes.
// Approach: each non-empty tower contributes 4v+2; subtract 2*min of shared faces with right/down neighbours.
// Time: O(n^2) | Space: O(1)
pub fn surface_area(g: &[Vec<i32>]) -> i32 {
    let mut s = 0;
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            let v = g[i][j];
            if v > 0 {
                s += 4 * v + 2;
            }
            if i + 1 < g.len() {
                s -= 2 * v.min(g[i + 1][j]);
            }
            if j + 1 < g[i].len() {
                s -= 2 * v.min(g[i][j + 1]);
            }
        }
    }
    s
}

fn main() {
    assert_eq!(surface_area(&[vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(surface_area(&[vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), 32);
    println!("ok");
}
