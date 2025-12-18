// LeetCode 172 - Factorial Trailing Zeroes
// Return the number of trailing zeroes in n!.
// Approach: Legendre's formula for the exponent of 5: n/5 + n/25 + n/125 + ...
// Time: O(log n), Space: O(1)
fn trailing_zeroes(mut n: u64) -> u64 {
    let mut c = 0;
    while n > 0 {
        n /= 5;
        c += n;
    }
    c
}

fn main() {
    assert_eq!(trailing_zeroes(5), 1);
    assert_eq!(trailing_zeroes(100), 24);
    assert_eq!(trailing_zeroes(0), 0);
    println!("ok");
}
