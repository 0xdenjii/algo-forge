// LeetCode 1584 - Min Cost to Connect All Points
// Connect all 2D points with minimum total Manhattan-distance edge cost.
// Approach: dense Prim's MST; the O(V^2) array version beats a heap on a complete graph.
// Time: O(n^2), Space: O(n)
pub fn min_cost_connect_points(points: &[[i32; 2]]) -> i64 {
    let n = points.len();
    if n == 0 {
        return 0;
    }
    let mut in_tree = vec![false; n];
    let mut best = vec![i64::MAX; n];
    best[0] = 0;
    let mut total = 0;
    for _ in 0..n {
        let u = (0..n).filter(|&i| !in_tree[i]).min_by_key(|&i| best[i]).unwrap();
        in_tree[u] = true;
        total += best[u];
        for v in 0..n {
            if !in_tree[v] {
                let d = ((points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs()) as i64;
                best[v] = best[v].min(d);
            }
        }
    }
    total
}

fn main() {
    let p = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]];
    assert_eq!(min_cost_connect_points(&p), 20);
    println!("ok");
}
