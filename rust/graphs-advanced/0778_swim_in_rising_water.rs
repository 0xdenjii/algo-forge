// LeetCode 778 - Swim in Rising Water
// grid[i][j] is an elevation; at time t you may swim between adjacent cells with
// elevation <= t. Return the least t to get from (0,0) to (n-1,n-1).
// Approach: minimax-path Dijkstra: the cost of a path is its maximum cell; a heap
// keyed by that max pops each cell with its optimal bottleneck.
// Time: O(n^2 log n), Space: O(n^2)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn swim_in_water(grid: &[Vec<i32>]) -> i32 {
    let n = grid.len();
    let mut seen = vec![vec![false; n]; n];
    let mut pq = BinaryHeap::from([Reverse((grid[0][0], 0usize, 0usize))]);
    seen[0][0] = true;
    while let Some(Reverse((t, r, c))) = pq.pop() {
        if r == n - 1 && c == n - 1 {
            return t;
        }
        let nbrs = [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)];
        for (nr, nc) in nbrs {
            if nr < n && nc < n && !seen[nr][nc] {
                seen[nr][nc] = true;
                pq.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
            }
        }
    }
    unreachable!("grid is connected")
}

fn main() {
    assert_eq!(swim_in_water(&[vec![0, 2], vec![1, 3]]), 3);
    let g = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    assert_eq!(swim_in_water(&g), 16);
    println!("ok");
}
