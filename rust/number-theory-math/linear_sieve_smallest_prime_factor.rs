// Classic - Linear Sieve (Euler sieve) with Smallest Prime Factor
// Compute all primes up to N and spf[i] (smallest prime factor) so any i <= N can be factorized
// in O(log i) by repeatedly dividing by spf.
// Approach: each composite i*p is marked exactly once, by its smallest prime p (stop when p > spf[i]).
// Time: O(N) sieve, O(log n) per factorization; Space: O(N)
struct Sieve {
    spf: Vec<u32>,
    primes: Vec<u32>,
}

impl Sieve {
    fn new(n: usize) -> Self {
        let mut spf = vec![0u32; n + 1];
        let mut primes = Vec::new();
        for i in 2..=n {
            if spf[i] == 0 {
                spf[i] = i as u32;
                primes.push(i as u32);
            }
            for &p in &primes {
                let m = i * p as usize;
                if p > spf[i] || m > n {
                    break;
                }
                spf[m] = p;
            }
        }
        Sieve { spf, primes }
    }

    fn factorize(&self, mut x: usize) -> Vec<(u32, u32)> {
        let mut out: Vec<(u32, u32)> = Vec::new();
        while x > 1 {
            let p = self.spf[x];
            match out.last_mut() {
                Some((q, e)) if *q == p => *e += 1,
                _ => out.push((p, 1)),
            }
            x /= p as usize;
        }
        out
    }
}

fn main() {
    let s = Sieve::new(100);
    assert_eq!(s.primes.len(), 25);
    assert_eq!(s.factorize(84), vec![(2, 2), (3, 1), (7, 1)]);
    assert_eq!(s.factorize(97), vec![(97, 1)]);
    assert!(s.factorize(1).is_empty());
    println!("ok");
}
