// Title: Linear Sieve of Eratosthenes + Smallest Prime Factor
// Problem: List all primes up to N and factorize any x <= N quickly.
// Approach: Linear sieve computing spf[x]; each composite is crossed out exactly once by its
// smallest prime factor. Factorize by repeatedly dividing by spf.
// Complexity: O(N) sieve, O(log x) per factorization; O(N) space.
pub fn linear_sieve(n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut spf = vec![0usize; n + 1];
    let mut primes = Vec::new();
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i;
            primes.push(i);
        }
        for &p in &primes {
            if p > spf[i] || i * p > n {
                break;
            }
            spf[i * p] = p;
        }
    }
    (primes, spf)
}

pub fn factorize(mut x: usize, spf: &[usize]) -> Vec<(usize, u32)> {
    let mut res: Vec<(usize, u32)> = Vec::new();
    while x > 1 {
        let p = spf[x];
        match res.last_mut() {
            Some((q, c)) if *q == p => *c += 1,
            _ => res.push((p, 1)),
        }
        x /= p;
    }
    res
}

fn main() {
    let (primes, spf) = linear_sieve(100);
    assert_eq!(primes.len(), 25);
    assert_eq!(factorize(84, &spf), vec![(2, 2), (3, 1), (7, 1)]);
    println!("{:?}", &primes[..10]);
}
