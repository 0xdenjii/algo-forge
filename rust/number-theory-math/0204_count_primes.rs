// LeetCode 204 - Count Primes
// Given n, return the number of primes strictly less than n.
// Approach: Sieve of Eratosthenes, crossing out multiples starting at p*p.
// Time: O(n log log n), Space: O(n)
fn count_primes(n: usize) -> usize {
    if n < 3 {
        return 0;
    }
    let mut is = vec![true; n];
    is[0] = false;
    is[1] = false;
    let mut p = 2;
    while p * p < n {
        if is[p] {
            for m in (p * p..n).step_by(p) {
                is[m] = false;
            }
        }
        p += 1;
    }
    is.iter().filter(|&&b| b).count()
}

fn main() {
    assert_eq!(count_primes(10), 4);
    assert_eq!(count_primes(0), 0);
    println!("{}", count_primes(1_000_000)); // 78498
}
