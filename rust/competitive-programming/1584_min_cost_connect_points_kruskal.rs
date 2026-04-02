// Problem 1584: Min Cost to Connect All Points
// Given 2D points, connect all of them so every pair is reachable, minimizing the total
// Manhattan distance of the chosen edges.
// Approach: Kruskal's MST: sort all O(n^2) edges, add each with DSU if it joins two trees.
// Complexity: O(n^2 log n) time, O(n^2) space.
fn find(p: &mut [usize], mut x: usize) -> usize {
    while p[x] != x {
        p[x] = p[p[x]];
        x = p[x];
    }
    x
}

pub fn min_cost_connect_points(pts: &[[i64; 2]]) -> i64 {
    let n = pts.len();
    let mut edges = Vec::with_capacity(n * n / 2);
    for i in 0..n {
        for j in i + 1..n {
            let w = (pts[i][0] - pts[j][0]).abs() + (pts[i][1] - pts[j][1]).abs();
            edges.push((w, i, j));
        }
    }
    edges.sort_unstable();
    let mut p: Vec<usize> = (0..n).collect();
    let (mut cost, mut used) = (0, 0);
    for (w, a, b) in edges {
        let (ra, rb) = (find(&mut p, a), find(&mut p, b));
        if ra != rb {
            p[ra] = rb;
            cost += w;
            used += 1;
            if used + 1 == n {
                break;
            }
        }
    }
    cost
}

fn main() {
    assert_eq!(min_cost_connect_points(&[[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]), 20);
    assert_eq!(min_cost_connect_points(&[[3, 12], [-2, 5], [-4, 1]]), 18);
    assert_eq!(min_cost_connect_points(&[[0, 0]]), 0);
    println!("ok");
}
