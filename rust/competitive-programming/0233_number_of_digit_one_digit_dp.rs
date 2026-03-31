// Problem 233: Number of Digit One
// Count the total number of times the digit 1 appears in all integers from 0 to n.
// Approach: Digit DP over the decimal digits with state (position, ones so far, tight);
// only non-tight states are memoized since tight states occur once per position.
// Complexity: O(D^2 * 10) time, O(D^2) space, where D is the number of digits.
fn go(i: usize, cnt: usize, tight: bool, d: &[u8], memo: &mut [Vec<Option<u64>>]) -> u64 {
    if i == d.len() {
        return cnt as u64;
    }
    if !tight {
        if let Some(v) = memo[i][cnt] {
            return v;
        }
    }
    let lim = if tight { d[i] } else { 9 };
    let r = (0..=lim)
        .map(|x| go(i + 1, cnt + (x == 1) as usize, tight && x == lim, d, memo))
        .sum();
    if !tight {
        memo[i][cnt] = Some(r);
    }
    r
}

pub fn count_digit_one(n: u64) -> u64 {
    let d: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
    let mut memo = vec![vec![None; d.len() + 1]; d.len()];
    go(0, 0, true, &d, &mut memo)
}

fn main() {
    assert_eq!(count_digit_one(13), 6);
    assert_eq!(count_digit_one(0), 0);
    let brute: u64 = (0..=2024u64).map(|x| x.to_string().matches('1').count() as u64).sum();
    assert_eq!(count_digit_one(2024), brute);
    println!("ok");
}
