// Classic - Intersection Point of Two Lines / Segments
// Given segments P1P2 and P3P4 in floating point, return their intersection point if it exists
// (None for parallel lines or when the crossing lies outside either segment).
// Approach: parametric form P1 + t*(P2-P1); solve with 2D cross products, check t,u in [0,1].
// Time: O(1) | Space: O(1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Sub for Pt {
    type Output = Pt;
    fn sub(self, o: Pt) -> Pt {
        Pt { x: self.x - o.x, y: self.y - o.y }
    }
}

fn cross(a: Pt, b: Pt) -> f64 {
    a.x * b.y - a.y * b.x
}

const EPS: f64 = 1e-12;

pub fn line_intersection(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> Option<Pt> {
    let (r, s) = (p2 - p1, p4 - p3);
    let denom = cross(r, s);
    if denom.abs() < EPS {
        return None;
    }
    let qp = p3 - p1;
    Some(Pt { x: p1.x + r.x * cross(qp, s) / denom, y: p1.y + r.y * cross(qp, s) / denom })
}

pub fn segment_intersection(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> Option<Pt> {
    let (r, s) = (p2 - p1, p4 - p3);
    let denom = cross(r, s);
    if denom.abs() < EPS {
        return None;
    }
    let qp = p3 - p1;
    let t = cross(qp, s) / denom;
    let u = cross(qp, r) / denom;
    ((-EPS..=1.0 + EPS).contains(&t) && (-EPS..=1.0 + EPS).contains(&u))
        .then(|| Pt { x: p1.x + t * r.x, y: p1.y + t * r.y })
}

fn main() {
    let p = |x, y| Pt { x, y };
    let i = segment_intersection(p(0., 0.), p(4., 4.), p(0., 4.), p(4., 0.)).unwrap();
    assert!((i.x - 2.0).abs() < 1e-9 && (i.y - 2.0).abs() < 1e-9);
    assert!(segment_intersection(p(0., 0.), p(1., 1.), p(3., 0.), p(4., -1.)).is_none());
    assert!(line_intersection(p(0., 0.), p(1., 1.), p(3., 0.), p(4., -1.)).is_some());
    assert!(line_intersection(p(0., 0.), p(1., 1.), p(0., 1.), p(1., 2.)).is_none());
    println!("{:?}", i);
}
