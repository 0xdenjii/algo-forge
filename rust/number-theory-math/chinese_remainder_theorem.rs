// Classic - Generalized Chinese Remainder Theorem
// Given congruences x = r_i (mod m_i) with moduli that need not be coprime, find the smallest
// non-negative x and the combined modulus lcm(m_i), or report that the system is inconsistent.
// Approach: merge congruences pairwise with extended Euclid; a merge fails iff
// (r2 - r1) is not divisible by gcd(m1, m2). Uses i128 to avoid overflow.
// Time: O(k log M), Space: O(1)
fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Returns Some((x, lcm)) with 0 <= x < lcm, or None if no solution.
fn crt(congruences: &[(i64, i64)]) -> Option<(i64, i64)> {
    let (mut r, mut m): (i128, i128) = (0, 1);
    for &(ri, mi) in congruences {
        let (ri, mi) = ((ri as i128).rem_euclid(mi as i128), mi as i128);
        let (g, p, _) = ext_gcd(m, mi);
        let diff = ri - r;
        if diff % g != 0 {
            return None;
        }
        let step = mi / g;
        let k = ((diff / g) % step * (p % step)).rem_euclid(step);
        r += m * k;
        m *= step;
        r = r.rem_euclid(m);
    }
    Some((r as i64, m as i64))
}

fn main() {
    // Sun Tzu: x = 2 mod 3, 3 mod 5, 2 mod 7 -> 23 mod 105
    assert_eq!(crt(&[(2, 3), (3, 5), (2, 7)]), Some((23, 105)));
    // Non-coprime moduli
    assert_eq!(crt(&[(2, 4), (4, 6)]), Some((10, 12)));
    assert_eq!(crt(&[(1, 4), (2, 6)]), None);
    println!("ok");
}
