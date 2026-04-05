// Title: Modular Arithmetic (fast power, inverse, nCr)
// Problem: Compute a^b mod p, modular inverses and binomial coefficients C(n, r) mod p for
// many queries with p = 1e9+7 prime.
// Approach: Binary exponentiation; Fermat inverse a^(p-2); precomputed factorials and
// inverse factorials.
// Complexity: O(log b) per pow; O(N) precompute, O(1) per nCr. Space O(N).
const MOD: u64 = 1_000_000_007;

pub fn pow_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    r
}

pub fn inv(a: u64) -> u64 {
    pow_mod(a, MOD - 2, MOD)
}

pub struct Comb {
    f: Vec<u64>,
    fi: Vec<u64>,
}

impl Comb {
    pub fn new(n: usize) -> Self {
        let mut f = vec![1u64; n + 1];
        for i in 1..=n {
            f[i] = f[i - 1] * i as u64 % MOD;
        }
        let mut fi = vec![1u64; n + 1];
        fi[n] = inv(f[n]);
        for i in (0..n).rev() {
            fi[i] = fi[i + 1] * (i as u64 + 1) % MOD;
        }
        Self { f, fi }
    }
    pub fn ncr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            0
        } else {
            self.f[n] * self.fi[r] % MOD * self.fi[n - r] % MOD
        }
    }
}

fn main() {
    assert_eq!(pow_mod(2, 10, MOD), 1024);
    assert_eq!(inv(2) * 2 % MOD, 1);
    let c = Comb::new(1000);
    assert_eq!(c.ncr(5, 2), 10);
    assert_eq!(c.ncr(10, 11), 0);
    println!("C(1000,500) mod p = {}", c.ncr(1000, 500));
}
