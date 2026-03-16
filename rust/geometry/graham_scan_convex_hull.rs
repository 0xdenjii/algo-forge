// Classic - Convex Hull (Graham Scan)
// Given points, return the vertices of their convex hull in counter-clockwise order,
// excluding collinear points on hull edges.
// Approach: pick the lowest (then leftmost) pivot, sort others by polar angle via cross product
// (ties by distance), then scan with a stack popping non-left turns.
// Time: O(n log n) | Space: O(n)
use std::cmp::Ordering;

type P = (i64, i64);

fn cross(o: P, a: P, b: P) -> i64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn d2(a: P, b: P) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

pub fn graham_scan(points: &[P]) -> Vec<P> {
    let mut pts = points.to_vec();
    pts.sort_unstable();
    pts.dedup();
    if pts.len() < 3 {
        return pts;
    }
    let pivot = *pts.iter().min_by_key(|p| (p.1, p.0)).unwrap();
    pts.retain(|&p| p != pivot);
    pts.sort_by(|&a, &b| match cross(pivot, a, b) {
        c if c > 0 => Ordering::Less,
        c if c < 0 => Ordering::Greater,
        _ => d2(pivot, a).cmp(&d2(pivot, b)),
    });
    let mut st = vec![pivot];
    for p in pts {
        while st.len() >= 2 && cross(st[st.len() - 2], st[st.len() - 1], p) <= 0 {
            st.pop();
        }
        st.push(p);
    }
    st
}

fn main() {
    let pts = [(0, 3), (1, 1), (2, 2), (4, 4), (0, 0), (1, 2), (3, 1), (3, 3), (2, 0)];
    let h = graham_scan(&pts);
    assert_eq!(h, vec![(0, 0), (2, 0), (3, 1), (4, 4), (0, 3)]);
    assert_eq!(graham_scan(&[(0, 0), (1, 1), (2, 2)]), vec![(0, 0), (2, 2)]);
    println!("{:?}", h);
}
