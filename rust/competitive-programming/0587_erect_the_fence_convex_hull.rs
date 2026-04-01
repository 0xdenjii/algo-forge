// Problem 587: Erect the Fence
// Given tree coordinates, return all trees lying on the perimeter of the minimal enclosing
// fence (the convex hull), including collinear points on its boundary.
// Approach: Andrew's monotone chain over sorted points, building lower then upper hulls; pop
// only on strict clockwise turns (cross < 0) so collinear boundary points are kept; dedupe.
// Complexity: O(n log n) time, O(n) space.
type P = (i64, i64);

fn cross(o: P, a: P, b: P) -> i64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn half_hull<'a>(pts: impl Iterator<Item = &'a P>) -> Vec<P> {
    let mut h: Vec<P> = Vec::new();
    for &p in pts {
        while h.len() >= 2 && cross(h[h.len() - 2], h[h.len() - 1], p) < 0 {
            h.pop();
        }
        h.push(p);
    }
    h
}

pub fn outer_trees(mut pts: Vec<P>) -> Vec<P> {
    pts.sort_unstable();
    pts.dedup();
    if pts.len() <= 2 {
        return pts;
    }
    let mut hull = half_hull(pts.iter());
    hull.extend(half_hull(pts.iter().rev()));
    hull.sort_unstable();
    hull.dedup();
    hull
}

fn main() {
    let r = outer_trees(vec![(1, 1), (2, 2), (2, 0), (2, 4), (3, 3), (4, 2)]);
    assert_eq!(r, vec![(1, 1), (2, 0), (2, 4), (3, 3), (4, 2)]);
    assert_eq!(outer_trees(vec![(1, 2), (2, 2), (4, 2)]), vec![(1, 2), (2, 2), (4, 2)]);
    let sq = outer_trees(vec![(0, 0), (0, 2), (2, 0), (2, 2), (1, 1), (1, 0)]);
    assert_eq!(sq, vec![(0, 0), (0, 2), (1, 0), (2, 0), (2, 2)]);
    println!("ok");
}
