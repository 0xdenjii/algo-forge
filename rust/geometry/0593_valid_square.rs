// LeetCode 593 - Valid Square
// Given four points in any order, decide whether they form a square
// with positive side length.
// Approach: sorted 6 pairwise squared distances must be 4 equal sides (>0) and 2 diagonals = 2*side.
// Time: O(1) | Space: O(1)
fn d2(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

pub fn valid_square(p: [(i64, i64); 4]) -> bool {
    let mut d = Vec::with_capacity(6);
    for i in 0..4 {
        for j in i + 1..4 {
            d.push(d2(p[i], p[j]));
        }
    }
    d.sort_unstable();
    d[0] > 0 && d[0] == d[3] && d[4] == d[5] && d[4] == 2 * d[0]
}

fn main() {
    assert!(valid_square([(0, 0), (1, 1), (1, 0), (0, 1)]));
    assert!(!valid_square([(0, 0), (1, 1), (1, 0), (0, 12)]));
    assert!(!valid_square([(0, 0); 4]));
    println!("ok");
}
