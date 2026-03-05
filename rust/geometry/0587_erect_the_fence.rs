// LeetCode 587 - Erect the Fence
// Given tree coordinates, return the trees on the perimeter of the minimal fence
// enclosing all trees (convex hull including collinear boundary points).
// Approach: monotone chain popping only on strict clockwise turns so collinear points stay; dedupe.
// Time: O(n log n) | Space: O(n)
fn cross(o: (i32, i32), a: (i32, i32), b: (i32, i32)) -> i64 {
    (a.0 - o.0) as i64 * (b.1 - o.1) as i64 - (a.1 - o.1) as i64 * (b.0 - o.0) as i64
}

pub fn outer_trees(mut pts: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    pts.sort_unstable();
    pts.dedup();
    if pts.len() <= 3 {
        return pts;
    }
    let mut hull: Vec<(i32, i32)> = Vec::new();
    for &p in &pts {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) < 0 {
            hull.pop();
        }
        hull.push(p);
    }
    let lower = hull.len();
    for &p in pts.iter().rev().skip(1) {
        while hull.len() > lower && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) < 0 {
            hull.pop();
        }
        hull.push(p);
    }
    hull.sort_unstable();
    hull.dedup();
    hull
}

fn main() {
    let r = outer_trees(vec![(1, 1), (2, 2), (2, 0), (2, 4), (3, 3), (4, 2)]);
    assert_eq!(r, vec![(1, 1), (2, 0), (2, 4), (3, 3), (4, 2)]);
    assert_eq!(outer_trees(vec![(1, 2), (2, 2), (4, 2), (3, 2)]).len(), 4);
    println!("{:?}", r);
}
