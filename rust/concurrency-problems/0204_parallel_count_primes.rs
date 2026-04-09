// LeetCode 204 (parallel variant) - Count Primes with a Segmented Sieve
// Count primes strictly less than n, splitting the work across threads.
// Approach: sieve base primes up to sqrt(n) once; then each scoped thread sieves its
// own contiguous segment [lo, hi) using the base primes. Segments are independent.
// Time: O(n log log n) work. Space: O(sqrt(n) + n / threads) per thread.
use std::thread;

fn simple_sieve(limit: usize) -> Vec<usize> {
    let mut is = vec![true; limit + 1];
    let mut ps = vec![];
    for i in 2..=limit {
        if is[i] {
            ps.push(i);
            let mut j = i * i;
            while j <= limit { is[j] = false; j += i; }
        }
    }
    ps
}

pub fn count_primes(n: usize, threads: usize) -> usize {
    if n < 3 { return 0; }
    let root = (n as f64).sqrt() as usize + 1;
    let base = simple_sieve(root);
    let seg = (n - 2).div_ceil(threads.max(1));
    thread::scope(|s| {
        let base = &base;
        (0..threads)
            .map(|t| {
                let lo = 2 + t * seg;
                let hi = (lo + seg).min(n);
                s.spawn(move || {
                    if lo >= hi { return 0; }
                    let mut is = vec![true; hi - lo];
                    for &p in base {
                        if p * p >= hi { break; }
                        let start = (p * p).max(lo.div_ceil(p) * p);
                        let mut j = start;
                        while j < hi { is[j - lo] = false; j += p; }
                    }
                    is.iter().filter(|&&b| b).count()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum()
    })
}

fn main() {
    assert_eq!(count_primes(10, 3), 4);
    assert_eq!(count_primes(0, 2), 0);
    assert_eq!(count_primes(2, 2), 0);
    assert_eq!(count_primes(3, 4), 1);
    assert_eq!(count_primes(1_000_000, 8), 78_498);
    println!("pi(10^7) = {}", count_primes(10_000_000, 8));
}
