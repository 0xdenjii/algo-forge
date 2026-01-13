// Classic - Segmented Sieve (SPOJ PRIME1 style)
// List all primes in [lo, hi] where hi can be up to ~10^12 but hi - lo is small (e.g. <= 10^6).
// Approach: sieve base primes up to sqrt(hi), then cross out their multiples inside the window,
// starting at max(p*p, first multiple >= lo).
// Time: O(sqrt(hi) log log hi + (hi - lo) log log hi), Space: O(sqrt(hi) + hi - lo)
fn simple_sieve(n: usize) -> Vec<u64> {
    let mut is = vec![true; n + 1];
    let mut primes = Vec::new();
    for i in 2..=n {
        if is[i] {
            primes.push(i as u64);
            for m in (i * i..=n).step_by(i) {
                is[m] = false;
            }
        }
    }
    primes
}

fn primes_in_range(lo: u64, hi: u64) -> Vec<u64> {
    if hi < 2 || lo > hi {
        return Vec::new();
    }
    let lo = lo.max(2);
    let base = simple_sieve((hi as f64).sqrt() as usize + 1);
    let mut is = vec![true; (hi - lo + 1) as usize];
    for &p in &base {
        let start = (p * p).max(lo.div_ceil(p) * p);
        let mut m = start;
        while m <= hi {
            is[(m - lo) as usize] = false;
            m += p;
        }
    }
    is.iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| lo + i as u64)
        .collect()
}

fn main() {
    assert_eq!(primes_in_range(1, 10), vec![2, 3, 5, 7]);
    assert_eq!(primes_in_range(3, 5), vec![3, 5]);
    assert_eq!(primes_in_range(1_000_000_000, 1_000_000_100), vec![1_000_000_007, 1_000_000_009, 1_000_000_021, 1_000_000_033, 1_000_000_087, 1_000_000_093, 1_000_000_097]);
    println!("ok");
}
