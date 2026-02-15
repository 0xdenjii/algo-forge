// LeetCode 902 - Numbers At Most N Given Digit Set
// Given a sorted set of digits (1-9), count positive integers <= n that can be
// written using only those digits (with repetition).
// Approach: digit DP: count all shorter numbers, then walk n's digits counting
// smaller choices at each position while staying tight.
// Time: O(log n * |D|), Space: O(log n)

pub fn at_most_n_given_digit_set(digits: &[char], n: u64) -> u64 {
    let s: Vec<u32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let d: Vec<u32> = digits.iter().map(|c| c.to_digit(10).unwrap()).collect();
    let k = d.len() as u64;
    let len = s.len();
    let mut total: u64 = (1..len).map(|l| k.pow(l as u32)).sum();
    for (i, &x) in s.iter().enumerate() {
        let smaller = d.iter().filter(|&&y| y < x).count() as u64;
        total += smaller * k.pow((len - i - 1) as u32);
        if !d.contains(&x) {
            return total;
        }
    }
    total + 1 // n itself is representable
}

fn main() {
    assert_eq!(at_most_n_given_digit_set(&['1', '3', '5', '7'], 100), 20);
    assert_eq!(at_most_n_given_digit_set(&['1', '4', '9'], 1_000_000_000), 29523);
    assert_eq!(at_most_n_given_digit_set(&['7'], 8), 1);
    println!("ok");
}
