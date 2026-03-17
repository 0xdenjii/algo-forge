// Classic - Polygon Clipping (Sutherland-Hodgman)
// Clip a subject polygon against a convex clip polygon (both CCW) and return the resulting polygon;
// e.g. compute the intersection area of two convex polygons.
// Approach: clip successively against each clip edge's half-plane, emitting inside points and
// edge/line intersections.
// Time: O(n * m) | Space: O(n + m)
type P = (f64, f64);

fn cross(o: P, a: P, b: P) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn intersect(a: P, b: P, c: P, d: P) -> P {
    let (r, s) = ((b.0 - a.0, b.1 - a.1), (d.0 - c.0, d.1 - c.1));
    let t = ((c.0 - a.0) * s.1 - (c.1 - a.1) * s.0) / (r.0 * s.1 - r.1 * s.0);
    (a.0 + t * r.0, a.1 + t * r.1)
}

pub fn clip(subject: &[P], clipper: &[P]) -> Vec<P> {
    let mut out = subject.to_vec();
    let m = clipper.len();
    for i in 0..m {
        if out.is_empty() {
            break;
        }
        let (c1, c2) = (clipper[i], clipper[(i + 1) % m]);
        let input = std::mem::take(&mut out);
        let n = input.len();
        for k in 0..n {
            let (cur, prev) = (input[k], input[(k + n - 1) % n]);
            let (cin, pin) = (cross(c1, c2, cur) >= 0.0, cross(c1, c2, prev) >= 0.0);
            if cin {
                if !pin {
                    out.push(intersect(prev, cur, c1, c2));
                }
                out.push(cur);
            } else if pin {
                out.push(intersect(prev, cur, c1, c2));
            }
        }
    }
    out
}

pub fn area(p: &[P]) -> f64 {
    let n = p.len();
    (0..n).map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1).sum::<f64>().abs() / 2.0
}

fn main() {
    let a = [(0., 0.), (4., 0.), (4., 4.), (0., 4.)];
    let b = [(2., 2.), (6., 2.), (6., 6.), (2., 6.)];
    let c = clip(&a, &b);
    assert!((area(&c) - 4.0).abs() < 1e-9);
    let far = [(10., 10.), (11., 10.), (11., 11.)];
    assert!(clip(&far, &a).is_empty());
    println!("{:?}", c);
}
