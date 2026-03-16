// Classic - Minimum Enclosing Circle (Welzl, iterative randomized incremental)
// Given points in the plane, find the smallest circle containing all of them
// (also LeetCode 1924 "Erect the Fence II").
// Approach: shuffle, then incrementally fix 1, 2 or 3 boundary points; expected linear time.
// Time: O(n) expected | Space: O(n)
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

type P = (f64, f64);
const EPS: f64 = 1e-9;

impl Circle {
    fn contains(&self, p: P) -> bool {
        (p.0 - self.x).hypot(p.1 - self.y) <= self.r + EPS
    }
    fn from2(a: P, b: P) -> Circle {
        let (x, y) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
        Circle { x, y, r: (a.0 - x).hypot(a.1 - y) }
    }
    fn from3(a: P, b: P, c: P) -> Circle {
        let (bx, by, cx, cy) = (b.0 - a.0, b.1 - a.1, c.0 - a.0, c.1 - a.1);
        let d = 2.0 * (bx * cy - by * cx);
        if d.abs() < EPS {
            // Collinear: the circle on the farthest pair.
            let cands = [Circle::from2(a, b), Circle::from2(a, c), Circle::from2(b, c)];
            return cands.into_iter().max_by(|p, q| p.r.total_cmp(&q.r)).unwrap();
        }
        let ux = (cy * (bx * bx + by * by) - by * (cx * cx + cy * cy)) / d;
        let uy = (bx * (cx * cx + cy * cy) - cx * (bx * bx + by * by)) / d;
        Circle { x: ux + a.0, y: uy + a.1, r: ux.hypot(uy) }
    }
}

fn shuffle(v: &mut [P], mut seed: u64) {
    for i in (1..v.len()).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        v.swap(i, (seed % (i as u64 + 1)) as usize);
    }
}

pub fn min_enclosing_circle(points: &[P]) -> Circle {
    let mut p = points.to_vec();
    shuffle(&mut p, 0x9E3779B97F4A7C15);
    let mut c = Circle { x: p[0].0, y: p[0].1, r: 0.0 };
    for i in 1..p.len() {
        if c.contains(p[i]) {
            continue;
        }
        c = Circle { x: p[i].0, y: p[i].1, r: 0.0 };
        for j in 0..i {
            if c.contains(p[j]) {
                continue;
            }
            c = Circle::from2(p[i], p[j]);
            for k in 0..j {
                if !c.contains(p[k]) {
                    c = Circle::from3(p[i], p[j], p[k]);
                }
            }
        }
    }
    c
}

fn main() {
    let c = min_enclosing_circle(&[(1., 1.), (2., 2.), (2., 0.), (2., 4.), (3., 3.), (4., 2.)]);
    assert!((c.x - 2.0).abs() < 1e-6 && (c.y - 2.0).abs() < 1e-6 && (c.r - 2.0).abs() < 1e-6);
    let c2 = min_enclosing_circle(&[(0., 0.), (4., 0.), (0., 3.)]);
    assert!((c2.r - 2.5).abs() < 1e-9);
    println!("{:?}", c);
}
