// LeetCode 50 - Pow(x, n)
// Implement pow(x, n) for f64 x and i32 n (n may be negative, including i32::MIN).
// Approach: binary exponentiation on |n| as i64; invert the result for negative n.
// Time: O(log n), Space: O(1)
fn my_pow(x: f64, n: i32) -> f64 {
    let mut e = (n as i64).abs();
    let (mut base, mut res) = (x, 1.0);
    while e > 0 {
        if e & 1 == 1 {
            res *= base;
        }
        base *= base;
        e >>= 1;
    }
    if n < 0 { 1.0 / res } else { res }
}

fn main() {
    assert!((my_pow(2.0, 10) - 1024.0).abs() < 1e-9);
    assert!((my_pow(2.0, -2) - 0.25).abs() < 1e-9);
    println!("{}", my_pow(2.1, 3));
}
