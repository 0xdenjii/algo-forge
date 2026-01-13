// Classic - Deterministic Miller-Rabin Primality Test (64-bit)
// Decide whether a u64 n is prime.
// Approach: write n-1 = d * 2^s; test the witness bases {2,3,5,7,11,13,17,19,23,29,31,37},
// which are proven deterministic for all n < 3.3 * 10^24. Use u128 for overflow-free mulmod.
// Time: O(k log^2 n) with k = 12 bases, Space: O(1)
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn pow_mod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mul_mod(r, a, m);
        }
        a = mul_mod(a, a, m);
        e >>= 1;
    }
    r
}

pub fn is_prime(n: u64) -> bool {
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for &p in &BASES {
        if n % p == 0 {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'witness: for &a in &BASES {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn main() {
    assert!(is_prime(2));
    assert!(is_prime(1_000_000_007));
    assert!(!is_prime(561)); // Carmichael number
    assert!(is_prime(18_446_744_073_709_551_557)); // largest 64-bit prime
    assert!(!is_prime(3_215_031_751)); // strong pseudoprime to bases 2,3,5,7
    println!("ok");
}
