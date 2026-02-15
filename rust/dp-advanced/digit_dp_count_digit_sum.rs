// Classic Digit DP - Count numbers in [0, N] whose digit sum equals S
// Given N up to 10^18 and a target digit sum S, count integers 0..=N with
// digit sum exactly S.
// Approach: memoized digit DP over (position, sum so far, tight flag); only the
// non-tight states are cached since tight states form a single path.
// Time: O(len * S * 10), Space: O(len * S)

pub fn count_digit_sum(n: u64, s: usize) -> u64 {
    let digits: Vec<usize> = n.to_string().bytes().map(|b| (b - b'0') as usize).collect();
    let mut memo = vec![vec![None; s + 1]; digits.len()];
    go(&digits, 0, 0, true, s, &mut memo)
}

fn go(d: &[usize], pos: usize, sum: usize, tight: bool, s: usize, memo: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if sum > s {
        return 0;
    }
    if pos == d.len() {
        return (sum == s) as u64;
    }
    if !tight {
        if let Some(v) = memo[pos][sum] {
            return v;
        }
    }
    let lim = if tight { d[pos] } else { 9 };
    let res = (0..=lim)
        .map(|x| go(d, pos + 1, sum + x, tight && x == lim, s, memo))
        .sum();
    if !tight {
        memo[pos][sum] = Some(res);
    }
    res
}

fn main() {
    let brute = |n: u64, s: usize| {
        (0..=n)
            .filter(|x| x.to_string().bytes().map(|b| (b - b'0') as usize).sum::<usize>() == s)
            .count() as u64
    };
    for (n, s) in [(100, 1), (12345, 15), (9999, 36), (500, 7)] {
        assert_eq!(count_digit_sum(n, s), brute(n, s));
    }
    println!("{}", count_digit_sum(1_000_000_000_000_000_000, 90));
}
