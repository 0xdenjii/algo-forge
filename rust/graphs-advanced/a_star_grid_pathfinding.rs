// A* Search on a Grid (classic)
// Find the shortest 4-directional path length between two cells of a grid where '#'
// marks walls, returning the path itself as well.
// Approach: A* with the Manhattan heuristic (admissible and consistent for unit
// 4-neighbor moves), so the first time the goal is popped its cost is optimal.
// Time: O(R*C log(R*C)) worst case, Space: O(R*C)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn a_star(grid: &[&str], start: (usize, usize), goal: (usize, usize)) -> Option<(usize, Vec<(usize, usize)>)> {
    let g: Vec<&[u8]> = grid.iter().map(|r| r.as_bytes()).collect();
    let (rows, cols) = (g.len(), g[0].len());
    let h = |(r, c): (usize, usize)| r.abs_diff(goal.0) + c.abs_diff(goal.1);
    let mut best = vec![vec![usize::MAX; cols]; rows];
    let mut parent = vec![vec![None; cols]; rows];
    best[start.0][start.1] = 0;
    let mut open = BinaryHeap::from([Reverse((h(start), 0usize, start))]);
    while let Some(Reverse((_, cost, cur))) = open.pop() {
        if cost > best[cur.0][cur.1] {
            continue;
        }
        if cur == goal {
            let mut path = vec![cur];
            let mut p = cur;
            while let Some(prev) = parent[p.0][p.1] {
                path.push(prev);
                p = prev;
            }
            path.reverse();
            return Some((cost, path));
        }
        let (r, c) = cur;
        let dirs = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in dirs {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nc < 0 || nr as usize >= rows || nc as usize >= cols {
                continue;
            }
            let nxt = (nr as usize, nc as usize);
            if g[nxt.0][nxt.1] == b'#' {
                continue;
            }
            let nc2 = cost + 1;
            if nc2 < best[nxt.0][nxt.1] {
                best[nxt.0][nxt.1] = nc2;
                parent[nxt.0][nxt.1] = Some(cur);
                open.push(Reverse((nc2 + h(nxt), nc2, nxt)));
            }
        }
    }
    None
}

fn main() {
    let grid = ["....#", ".##.#", "...#.", "#....", "....."];
    let (d, path) = a_star(&grid, (0, 0), (2, 4)).unwrap();
    assert_eq!(d, 8);
    assert_eq!(path.len(), 9);
    assert!(a_star(&[".#", "#."], (0, 0), (1, 1)).is_none());
    println!("ok");
}
