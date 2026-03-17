// Classic - Point in Polygon (Ray Casting with Boundary Detection)
// Given a simple polygon and a query point, classify the point as Inside, Outside or OnBoundary.
// Approach: check every edge for boundary membership; otherwise cast a ray to +x and count
// crossings using a half-open rule on y to handle vertices correctly. Exact integer arithmetic.
// Time: O(n) per query | Space: O(1)
#[derive(Debug, PartialEq, Eq)]
pub enum Location {
    Inside,
    Outside,
    OnBoundary,
}

type P = (i64, i64);

fn cross(o: P, a: P, b: P) -> i64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn on_segment(a: P, b: P, q: P) -> bool {
    cross(a, b, q) == 0
        && q.0 >= a.0.min(b.0) && q.0 <= a.0.max(b.0)
        && q.1 >= a.1.min(b.1) && q.1 <= a.1.max(b.1)
}

pub fn locate(poly: &[P], q: P) -> Location {
    let n = poly.len();
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if on_segment(a, b, q) {
            return Location::OnBoundary;
        }
        // Edge straddles the horizontal line y = q.1 (half-open interval).
        if (a.1 > q.1) != (b.1 > q.1) {
            // Is the crossing strictly to the right of q? Orientation sign test, oriented upward.
            let c = cross(a, b, q);
            if (b.1 > a.1 && c > 0) || (b.1 < a.1 && c < 0) {
                inside = !inside;
            }
        }
    }
    if inside { Location::Inside } else { Location::Outside }
}

fn main() {
    let poly = [(0, 0), (10, 0), (10, 10), (5, 5), (0, 10)];
    assert_eq!(locate(&poly, (2, 2)), Location::Inside);
    assert_eq!(locate(&poly, (5, 8)), Location::Outside);
    assert_eq!(locate(&poly, (10, 5)), Location::OnBoundary);
    assert_eq!(locate(&poly, (5, 5)), Location::OnBoundary);
    assert_eq!(locate(&poly, (-1, 5)), Location::Outside);
    println!("ok");
}
