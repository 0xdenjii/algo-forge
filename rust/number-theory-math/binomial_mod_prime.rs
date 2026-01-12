// Classic - Binomial Coefficients modulo a Prime (factorial tables + Lucas theorem)
// Answer many queries C(n, k) mod p. For n < table size use precomputed factorials and inverse
// factorials (Fermat inverse); for huge n with small p use Lucas: C(n,k) = prod C(n_i, k_i) over base-p digits.
// Approach: fact[i], inv_fact[i] built in O(N); inv_fact[N] = fact[N]^(p-2), then walk downward.
// Time: O(N + log p) precompute, O(1) per query (O(log_p n) with Lucas); Space: O(N)
struct Binom {
    p: u64,
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
}

fn pow_mod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

impl Binom {
    /// `n` must be < p for the inverses to exist.
    fn new(n: usize, p: u64) -> Self {
        let mut fact = vec![1u64; n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * i as u64 % p;
        }
        let mut inv_fact = vec![1u64; n + 1];
        inv_fact[n] = pow_mod(fact[n], p - 2, p);
        for i in (1..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * i as u64 % p;
        }
        Binom { p, fact, inv_fact }
    }

    fn c(&self, n: u64, k: u64) -> u64 {
        if k > n {
            return 0;
        }
        let (n, k) = (n as usize, k as usize);
        self.fact[n] * self.inv_fact[k] % self.p * self.inv_fact[n - k] % self.p
    }

    /// Lucas theorem; requires the tables to cover 0..p.
    fn lucas(&self, mut n: u64, mut k: u64) -> u64 {
        let mut r = 1;
        while k > 0 {
            r = r * self.c(n % self.p, k % self.p) % self.p;
            n /= self.p;
            k /= self.p;
        }
        r
    }
}

fn main() {
    let b = Binom::new(1000, 1_000_000_007);
    assert_eq!(b.c(5, 2), 10);
    assert_eq!(b.c(10, 11), 0);
    assert_eq!(b.c(1000, 500), 159_835_829);

    let small = Binom::new(12, 13);
    assert_eq!(small.lucas(1000, 300), {
        // Lucas digits: 1000 = (5,11,12)_13, 300 = (1,10,1)_13
        small.c(5, 1) * small.c(11, 10) % 13 * small.c(12, 1) % 13
    });
    println!("ok");
}
