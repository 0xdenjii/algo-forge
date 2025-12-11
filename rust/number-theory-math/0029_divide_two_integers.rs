// LeetCode 29 - Divide Two Integers
// Divide two i32 values without using *, / or %; truncate toward zero.
// If the quotient overflows i32 (i32::MIN / -1), return i32::MAX.
// Approach: work on i64 magnitudes; from bit 31 down, subtract divisor << shift when it fits.
// Time: O(32) = O(log n), Space: O(1)
fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }
    let negative = (dividend < 0) != (divisor < 0);
    let (mut a, b) = ((dividend as i64).abs(), (divisor as i64).abs());
    let mut q: i64 = 0;
    for shift in (0..32).rev() {
        if (a >> shift) >= b {
            a -= b << shift;
            q += 1 << shift;
        }
    }
    (if negative { -q } else { q }) as i32
}

fn main() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
    assert_eq!(divide(i32::MIN, 1), i32::MIN);
    assert_eq!(divide(i32::MIN, -1), i32::MAX);
    println!("ok");
}
