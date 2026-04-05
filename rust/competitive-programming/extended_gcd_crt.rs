// Title: Extended Euclid and Chinese Remainder Theorem
// Problem: Given congruences x = r_i (mod m_i) with arbitrary (not necessarily coprime)
// moduli, find the smallest non-negative x and the combined modulus, or report no solution.
// Approach: ext_gcd gives Bezout coefficients; merge congruences pairwise using i128.
// Complexity: O(k log M) time, O(1) extra space.
pub fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

pub fn crt(rs: &[i64], ms: &[i64]) -> Option<(i64, i64)> {
    let (mut r, mut m): (i128, i128) = (0, 1);
    for (&ri, &mi) in rs.iter().zip(ms) {
        let (ri, mi) = (ri as i128, mi as i128);
        let (g, p, _) = ext_gcd(m, mi);
        if (ri - r) % g != 0 {
            return None;
        }
        let mg = mi / g;
        let t = (((ri - r) / g % mg) * (p % mg) % mg + mg) % mg;
        let l = m * mg;
        r = ((r + m * t) % l + l) % l;
        m = l;
    }
    Some((r as i64, m as i64))
}

fn main() {
    assert_eq!(crt(&[2, 3, 2], &[3, 5, 7]), Some((23, 105)));
    assert_eq!(crt(&[1, 2], &[4, 6]), None);
    assert_eq!(crt(&[3, 5], &[4, 6]), Some((11, 12)));
    println!("ok");
}
