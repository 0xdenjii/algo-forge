// Classic - Bloom Filter
// Probabilistic set membership: insert(x) and might_contain(x) with no false negatives and a
// tunable false-positive rate p, sized for an expected n items.
// Approach: bit array of m = -n ln p / (ln 2)^2 bits and k = (m/n) ln 2 hash functions,
// derived by double hashing h1 + i*h2 from two seeded std hashers (Kirsch-Mitzenmacher).
// Time: O(k) per op. Space: O(m) bits.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: Vec<u64>,
    m: u64,
    k: u32,
}

impl BloomFilter {
    pub fn new(expected_items: usize, fp_rate: f64) -> Self {
        let n = expected_items.max(1) as f64;
        let ln2 = std::f64::consts::LN_2;
        let m = (-(n * fp_rate.ln()) / (ln2 * ln2)).ceil().max(64.0) as u64;
        let k = ((m as f64 / n) * ln2).round().max(1.0) as u32;
        Self { bits: vec![0; ((m + 63) / 64) as usize], m, k }
    }

    fn hashes<T: Hash>(item: &T) -> (u64, u64) {
        let mut a = DefaultHasher::new();
        0xA5u8.hash(&mut a);
        item.hash(&mut a);
        let mut b = DefaultHasher::new();
        0x5Au8.hash(&mut b);
        item.hash(&mut b);
        (a.finish(), b.finish() | 1)
    }

    fn positions<T: Hash>(&self, item: &T) -> impl Iterator<Item = u64> {
        let (h1, h2) = Self::hashes(item);
        let m = self.m;
        (0..self.k as u64).map(move |i| h1.wrapping_add(i.wrapping_mul(h2)) % m)
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        let pos: Vec<u64> = self.positions(item).collect();
        for p in pos {
            self.bits[(p / 64) as usize] |= 1 << (p % 64);
        }
    }

    pub fn might_contain<T: Hash>(&self, item: &T) -> bool {
        self.positions(item).all(|p| self.bits[(p / 64) as usize] >> (p % 64) & 1 == 1)
    }
}

fn main() {
    let mut bf = BloomFilter::new(10_000, 0.01);
    for i in 0..10_000u32 {
        bf.insert(&i);
    }
    assert!((0..10_000u32).all(|i| bf.might_contain(&i)), "no false negatives allowed");
    let fps = (10_000..110_000u32).filter(|i| bf.might_contain(i)).count();
    let rate = fps as f64 / 100_000.0;
    assert!(rate < 0.02, "false positive rate too high: {rate}");
    println!("bloom filter ok: m={} bits, k={}, observed fp rate {:.4}", bf.m, bf.k, rate);
}
