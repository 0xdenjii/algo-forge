// LeetCode 1631 - Path With Minimum Effort
// Moving between adjacent cells costs |height difference|; a route's effort is its
// maximum step cost. Return the minimum effort from top-left to bottom-right.
// Approach: Kruskal-style union-find: sort all adjacent-cell edges by weight, union
// until start and end are connected; the last weight added is the answer.
// Time: O(R*C log(R*C)), Space: O(R*C)
fn find(p: &mut [usize], mut x: usize) -> usize {
    while p[x] != x {
        p[x] = p[p[x]];
        x = p[x];
    }
    x
}

pub fn minimum_effort_path(h: &[Vec<i32>]) -> i32 {
    let (rows, cols) = (h.len(), h[0].len());
    let id = |r: usize, c: usize| r * cols + c;
    let mut edges = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if r + 1 < rows {
                edges.push(((h[r][c] - h[r + 1][c]).abs(), id(r, c), id(r + 1, c)));
            }
            if c + 1 < cols {
                edges.push(((h[r][c] - h[r][c + 1]).abs(), id(r, c), id(r, c + 1)));
            }
        }
    }
    edges.sort_unstable();
    let mut p: Vec<usize> = (0..rows * cols).collect();
    let (s, t) = (0, rows * cols - 1);
    if s == t {
        return 0;
    }
    for (w, a, b) in edges {
        let (ra, rb) = (find(&mut p, a), find(&mut p, b));
        if ra != rb {
            p[ra] = rb;
        }
        if find(&mut p, s) == find(&mut p, t) {
            return w;
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(minimum_effort_path(&[vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]), 2);
    assert_eq!(minimum_effort_path(&[vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]), 1);
    assert_eq!(minimum_effort_path(&[vec![7]]), 0);
    println!("ok");
}
