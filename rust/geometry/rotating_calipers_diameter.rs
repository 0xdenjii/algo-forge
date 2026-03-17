// Classic - Diameter of a Point Set (Rotating Calipers)
// Given points, find the largest squared distance between any two of them (farthest pair).
// Approach: build the convex hull (monotone chain), then rotate antipodal pointers around it;
// for each edge advance the opposite pointer while triangle area keeps increasing.
// Time: O(n log n) | Space: O(n)
type P = (i64, i64);

fn cross(o: P, a: P, b: P) -> i64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn d2(a: P, b: P) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn hull(mut p: Vec<P>) -> Vec<P> {
    p.sort_unstable();
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let mut h: Vec<P> = Vec::with_capacity(2 * p.len());
    for pass in 0..2 {
        let start = h.len();
        let iter: Box<dyn Iterator<Item = &P>> = if pass == 0 { Box::new(p.iter()) } else { Box::new(p.iter().rev()) };
        for &q in iter {
            while h.len() >= start + 2 && cross(h[h.len() - 2], h[h.len() - 1], q) <= 0 {
                h.pop();
            }
            h.push(q);
        }
        h.pop();
    }
    h
}

pub fn diameter_squared(points: &[P]) -> i64 {
    let h = hull(points.to_vec());
    let n = h.len();
    match n {
        0 | 1 => return 0,
        2 => return d2(h[0], h[1]),
        _ => {}
    }
    let mut best = 0;
    let mut j = 1;
    for i in 0..n {
        let ni = (i + 1) % n;
        while cross(h[i], h[ni], h[(j + 1) % n]).abs() > cross(h[i], h[ni], h[j]).abs() {
            j = (j + 1) % n;
        }
        best = best.max(d2(h[i], h[j])).max(d2(h[ni], h[j]));
    }
    best
}

fn main() {
    let pts = [(0, 0), (4, 0), (4, 3), (0, 3), (2, 1), (1, 2)];
    assert_eq!(diameter_squared(&pts), 25);
    assert_eq!(diameter_squared(&[(0, 0), (5, 5)]), 50);
    assert_eq!(diameter_squared(&[(0, 0), (1, 0), (7, 0)]), 49);
    println!("ok");
}
