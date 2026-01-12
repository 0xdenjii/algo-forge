// Classic - Euler's Totient Function (single value and sieve)
// phi(n) counts integers in [1, n] coprime to n. Compute phi(n) for one n, and phi(1..=N) for all.
// Approach: single value: trial-divide primes p and apply phi *= (1 - 1/p).
// Sieve: start phi[i] = i; for each prime p, for multiples m: phi[m] -= phi[m] / p.
// Time: O(sqrt n) single, O(N log log N) sieve; Space: O(1) / O(N)
fn phi(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn phi_sieve(n: usize) -> Vec<usize> {
    let mut phi: Vec<usize> = (0..=n).collect();
    for p in 2..=n {
        if phi[p] == p {
            // p is prime (untouched so far)
            for m in (p..=n).step_by(p) {
                phi[m] -= phi[m] / p;
            }
        }
    }
    phi
}

fn main() {
    assert_eq!(phi(1), 1);
    assert_eq!(phi(9), 6);
    assert_eq!(phi(36), 12);
    assert_eq!(phi(1_000_000_007), 1_000_000_006);
    let s = phi_sieve(100);
    assert!((1..=100).all(|i| s[i] as u64 == phi(i as u64)));
    println!("ok");
}
