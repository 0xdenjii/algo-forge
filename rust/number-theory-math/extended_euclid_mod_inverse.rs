// Classic - Extended Euclidean Algorithm and Modular Inverse
// Given a, b find g = gcd(a, b) and integers x, y with a*x + b*y = g.
// Use it to compute the modular inverse of a mod m (exists iff gcd(a, m) = 1).
// Approach: iterative extended Euclid maintaining Bezout coefficients.
// Time: O(log min(a, b)), Space: O(1)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64);
    let (mut old_t, mut t) = (0i64, 1i64);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r, old_s, old_t)
}

fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = ext_gcd(a.rem_euclid(m), m);
    (g == 1).then(|| x.rem_euclid(m))
}

fn main() {
    let (g, x, y) = ext_gcd(240, 46);
    assert_eq!(g, 2);
    assert_eq!(240 * x + 46 * y, 2);
    assert_eq!(mod_inverse(3, 11), Some(4));
    assert_eq!(mod_inverse(6, 9), None);
    assert_eq!(mod_inverse(-3, 11), Some(7));
    println!("ok");
}
