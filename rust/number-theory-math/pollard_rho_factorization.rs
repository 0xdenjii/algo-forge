// Classic - Integer Factorization with Pollard's Rho (Brent variant)
// Factor a u64 n into its prime factors (sorted, with multiplicity).
// Approach: Miller-Rabin to detect primes; otherwise find a nontrivial divisor with Pollard rho
// using f(x) = x^2 + c and Brent cycle detection with batched gcd, then recurse on both parts.
// Time: expected O(n^(1/4) polylog n), Space: O(log n)
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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn is_prime(n: u64) -> bool {
    const B: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for &p in &B {
        if n % p == 0 {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    B.iter().all(|&a| {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            return true;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                return true;
            }
        }
        false
    })
}

fn rho(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }
    let mut c = 1u64;
    loop {
        let f = |x: u64| (mul_mod(x, x, n) + c) % n;
        let (mut y, mut r, mut q, mut g) = (2u64, 1u64, 1u64, 1u64);
        let (mut x, mut ys) = (0u64, 0u64);
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..(128.min(r - k)) {
                    y = f(y);
                    q = mul_mod(q, x.abs_diff(y), n);
                }
                g = gcd(q, n);
                k += 128;
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n {
            return g;
        }
        c += 1; // retry with a different polynomial
    }
}

fn factor(n: u64, out: &mut Vec<u64>) {
    if n == 1 {
        return;
    }
    if is_prime(n) {
        out.push(n);
        return;
    }
    let d = rho(n);
    factor(d, out);
    factor(n / d, out);
}

pub fn factorize(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    factor(n, &mut v);
    v.sort_unstable();
    v
}

fn main() {
    assert_eq!(factorize(1), Vec::<u64>::new());
    assert_eq!(factorize(360), vec![2, 2, 2, 3, 3, 5]);
    assert_eq!(factorize(600_851_475_143), vec![71, 839, 1471, 6857]);
    assert_eq!(factorize(1_000_000_007 * 998_244_353), vec![998_244_353, 1_000_000_007]);
    println!("ok");
}
