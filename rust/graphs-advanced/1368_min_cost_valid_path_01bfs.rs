// LeetCode 1368 - Minimum Cost to Make at Least One Valid Path in a Grid
// Each cell has an arrow (1 right, 2 left, 3 down, 4 up). Changing a cell's arrow costs
// 1. Return the min cost so that following arrows leads from (0,0) to (m-1,n-1).
// Approach: 0-1 BFS with a deque: following the arrow costs 0 (push front), any other
// direction costs 1 (push back).
// Time: O(m*n), Space: O(m*n)
use std::collections::VecDeque;

pub fn min_cost(grid: &[Vec<u8>]) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    let mut dist = vec![vec![u32::MAX; n]; m];
    dist[0][0] = 0;
    let mut dq = VecDeque::from([(0usize, 0usize)]);
    while let Some((r, c)) = dq.pop_front() {
        let d = dist[r][c];
        for (i, &(dr, dc)) in dirs.iter().enumerate() {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let w = u32::from(grid[r][c] as usize != i + 1);
            if d + w < dist[nr][nc] {
                dist[nr][nc] = d + w;
                if w == 0 { dq.push_front((nr, nc)) } else { dq.push_back((nr, nc)) }
            }
        }
    }
    dist[m - 1][n - 1]
}

fn main() {
    let g = vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 1, 1, 1], vec![2, 2, 2, 2]];
    assert_eq!(min_cost(&g), 3);
    assert_eq!(min_cost(&[vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]), 0);
    assert_eq!(min_cost(&[vec![1, 2], vec![4, 3]]), 1);
    println!("ok");
}
