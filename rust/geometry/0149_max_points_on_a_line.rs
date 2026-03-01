// LeetCode 149 - Max Points on a Line
// Given integer points, return the maximum number of points lying on one straight line.
// Approach: for each anchor, normalize direction (dx,dy) by gcd and sign, count in a HashMap.
// Time: O(n^2) | Space: O(n)
use std::collections::HashMap;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

pub fn max_points(points: &[(i64, i64)]) -> usize {
    let n = points.len();
    if n <= 2 {
        return n;
    }
    let mut best = 0;
    for i in 0..n {
        let mut map: HashMap<(i64, i64), usize> = HashMap::new();
        let (mut dup, mut local) = (0, 0);
        for j in i + 1..n {
            let (mut dx, mut dy) = (points[j].0 - points[i].0, points[j].1 - points[i].1);
            if dx == 0 && dy == 0 {
                dup += 1;
                continue;
            }
            let g = gcd(dx, dy);
            dx /= g;
            dy /= g;
            if dx < 0 || (dx == 0 && dy < 0) {
                dx = -dx;
                dy = -dy;
            }
            let c = map.entry((dx, dy)).or_insert(0);
            *c += 1;
            local = local.max(*c);
        }
        best = best.max(local + dup + 1);
    }
    best
}

fn main() {
    assert_eq!(max_points(&[(1, 1), (2, 2), (3, 3)]), 3);
    assert_eq!(max_points(&[(1, 1), (3, 2), (5, 3), (4, 1), (2, 3), (1, 4)]), 4);
    println!("ok");
}
