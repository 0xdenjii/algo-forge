// Classic - Closest Pair of Points
// Given n points in the plane, find the minimum Euclidean distance between any two.
// Approach: divide and conquer on x-sorted points; merge by y (merge-sort style) and scan the
// vertical strip, comparing each point with following points whose y-gap is below the best.
// Time: O(n log n) | Space: O(n)
type P = (f64, f64);

fn dist(a: P, b: P) -> f64 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

fn rec(p: &mut [P]) -> f64 {
    let n = p.len();
    if n <= 3 {
        let mut best = f64::INFINITY;
        for i in 0..n {
            for j in i + 1..n {
                best = best.min(dist(p[i], p[j]));
            }
        }
        p.sort_by(|a, b| a.1.total_cmp(&b.1));
        return best;
    }
    let mid = n / 2;
    let mid_x = p[mid].0;
    let (l, r) = p.split_at_mut(mid);
    let mut d = rec(l).min(rec(r));
    // Merge halves by y.
    let mut merged = Vec::with_capacity(n);
    let (mut i, mut j) = (0, mid);
    while i < mid || j < n {
        if j >= n || (i < mid && p[i].1 <= p[j].1) {
            merged.push(p[i]);
            i += 1;
        } else {
            merged.push(p[j]);
            j += 1;
        }
    }
    p.copy_from_slice(&merged);
    let strip: Vec<P> = p.iter().copied().filter(|q| (q.0 - mid_x).abs() < d).collect();
    for a in 0..strip.len() {
        for b in a + 1..strip.len() {
            if strip[b].1 - strip[a].1 >= d {
                break;
            }
            d = d.min(dist(strip[a], strip[b]));
        }
    }
    d
}

pub fn closest_pair(points: &[P]) -> f64 {
    let mut p = points.to_vec();
    p.sort_by(|a, b| a.0.total_cmp(&b.0));
    rec(&mut p)
}

fn main() {
    let pts = [(2., 3.), (12., 30.), (40., 50.), (5., 1.), (12., 10.), (3., 4.)];
    let d = closest_pair(&pts);
    assert!((d - 2f64.sqrt()).abs() < 1e-9);
    // Cross-check against brute force on a deterministic pseudo-random set.
    let mut seed = 12345u64;
    let mut rnd = || { seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (seed >> 33) as f64 / 1e5 };
    let many: Vec<P> = (0..500).map(|_| (rnd(), rnd())).collect();
    let mut brute = f64::INFINITY;
    for i in 0..many.len() {
        for j in i + 1..many.len() {
            brute = brute.min(dist(many[i], many[j]));
        }
    }
    assert!((closest_pair(&many) - brute).abs() < 1e-9);
    println!("{d}");
}
