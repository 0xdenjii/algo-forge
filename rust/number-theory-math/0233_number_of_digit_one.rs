// LeetCode 233 - Number of Digit One
// Count the total number of digit 1 appearing in all non-negative integers <= n.
// Approach: for each place value m, split n into high / cur / low around that digit;
// ones at that place = high*m + (m if cur > 1, low+1 if cur == 1, 0 otherwise).
// Time: O(log n), Space: O(1)
fn count_digit_one(n: i64) -> i64 {
    let (mut count, mut m) = (0, 1i64);
    while m <= n {
        let (high, cur, low) = (n / (m * 10), (n / m) % 10, n % m);
        count += high * m
            + match cur {
                0 => 0,
                1 => low + 1,
                _ => m,
            };
        m *= 10;
    }
    count
}

fn main() {
    assert_eq!(count_digit_one(13), 6);
    assert_eq!(count_digit_one(0), 0);
    assert_eq!(count_digit_one(100), 21);
    println!("ok");
}
