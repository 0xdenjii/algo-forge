// LeetCode 279 - Perfect Squares
// Return the least number of perfect square numbers that sum to n.
// Approach: Lagrange: answer <= 4. Answer is 1 if n is square, 2 if n - i^2 is square for some i,
// 4 iff n = 4^a (8b + 7) (Legendre's three-square theorem), else 3.
// Time: O(sqrt n), Space: O(1)
fn is_square(n: u32) -> bool {
    let r = (n as f64).sqrt() as u32;
    (r.saturating_sub(1)..=r + 1).any(|x| x * x == n)
}

fn num_squares(mut n: u32) -> u32 {
    if is_square(n) {
        return 1;
    }
    let mut i = 1;
    while i * i <= n {
        if is_square(n - i * i) {
            return 2;
        }
        i += 1;
    }
    while n % 4 == 0 {
        n /= 4;
    }
    if n % 8 == 7 { 4 } else { 3 }
}

fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
    assert_eq!(num_squares(7), 4);
    assert_eq!(num_squares(16), 1);
    println!("ok");
}
