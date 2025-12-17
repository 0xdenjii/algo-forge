// LeetCode 149 - Max Points on a Line
// Given distinct integer points, return the maximum number of points lying on one straight line.
// Approach: for each anchor point, normalize the direction (dx, dy) by gcd and a canonical sign,
// then count identical directions in a HashMap. Exact integer arithmetic avoids float slopes.
// Time: O(n^2 log C), Space: O(n)
use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn max_points(p: &[[i32; 2]]) -> usize {
    let mut best = p.len().min(1);
    for i in 0..p.len() {
        let mut dirs: HashMap<(i32, i32), usize> = HashMap::new();
        for j in i + 1..p.len() {
            let (mut dx, mut dy) = (p[j][0] - p[i][0], p[j][1] - p[i][1]);
            let g = gcd(dx, dy);
            dx /= g;
            dy /= g;
            if dx < 0 || (dx == 0 && dy < 0) {
                dx = -dx;
                dy = -dy;
            }
            let e = dirs.entry((dx, dy)).or_insert(1); // anchor itself
            *e += 1;
            best = best.max(*e);
        }
    }
    best
}

fn main() {
    assert_eq!(max_points(&[[1, 1], [2, 2], [3, 3]]), 3);
    assert_eq!(max_points(&[[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]), 4);
    assert_eq!(max_points(&[[0, 0]]), 1);
    println!("ok");
}
