// Classic - Counting Inversions with a Fenwick Tree (SPOJ INVCNT)
// Problem: Count the pairs i < j with a[i] > a[j].
// Approach: Compress the values to ranks and scan left to right. The number
//   of earlier elements greater than a[j] is j - #earlier elements <= a[j],
//   and the Fenwick tree counts the latter.
// Complexity: O(n log n) time, O(n) space.

struct Fenwick(Vec<u32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize) {
        while i < self.0.len() {
            self.0[i] += 1;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> u64 {
        let mut s = 0u64;
        while i > 0 {
            s += self.0[i] as u64;
            i &= i - 1;
        }
        s
    }
}

fn count_inversions(a: &[i64]) -> u64 {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = Fenwick::new(sorted.len());
    let mut inv = 0;
    for (j, x) in a.iter().enumerate() {
        let r = sorted.binary_search(x).unwrap() + 1;
        inv += j as u64 - bit.prefix(r);
        bit.add(r);
    }
    inv
}

fn main() {
    assert_eq!(count_inversions(&[3, 1, 2]), 2);
    assert_eq!(count_inversions(&[2, 3, 8, 6, 1]), 5);
    assert_eq!(count_inversions(&[2, 4, 1, 3, 5]), 3);
    assert_eq!(count_inversions(&[1, 1, 1]), 0);

    // Randomized check against O(n^2) (deterministic LCG).
    let mut seed: u64 = 7;
    let a: Vec<i64> = (0..300)
        .map(|_| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((seed >> 33) % 50) as i64
        })
        .collect();
    let mut naive = 0u64;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] > a[j] {
                naive += 1;
            }
        }
    }
    assert_eq!(count_inversions(&a), naive);
    println!("inversion count ok");
}
