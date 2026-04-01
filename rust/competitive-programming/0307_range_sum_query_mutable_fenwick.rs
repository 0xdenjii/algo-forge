// Problem 307: Range Sum Query - Mutable (Fenwick Tree / BIT)
// Support point updates a[i] += d and range sum queries on an array; also used to count
// inversions in a permutation.
// Approach: Binary Indexed Tree where node i stores the sum over (i - lowbit(i), i].
// Complexity: O(log n) per operation, O(n) build, O(n) space.
pub struct Fenwick {
    t: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self { t: vec![0; n + 1] }
    }
    pub fn from_slice(a: &[i64]) -> Self {
        let n = a.len();
        let mut t = vec![0; n + 1];
        for i in 1..=n {
            t[i] += a[i - 1];
            let j = i + (i & i.wrapping_neg());
            if j <= n {
                t[j] += t[i];
            }
        }
        Self { t }
    }
    pub fn add(&mut self, i: usize, d: i64) {
        let mut i = i + 1;
        while i < self.t.len() {
            self.t[i] += d;
            i += i & i.wrapping_neg();
        }
    }
    /// Sum of a[0..i).
    pub fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.t[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    /// Sum of a[l..r).
    pub fn range(&self, l: usize, r: usize) -> i64 {
        self.prefix(r) - self.prefix(l)
    }
}

pub fn count_inversions(a: &[usize]) -> i64 {
    let n = a.iter().max().map_or(0, |m| m + 1);
    let mut f = Fenwick::new(n);
    let mut inv = 0;
    for (seen, &x) in a.iter().enumerate() {
        inv += seen as i64 - f.prefix(x + 1);
        f.add(x, 1);
    }
    inv
}

fn main() {
    let mut f = Fenwick::from_slice(&[1, 3, 5]);
    assert_eq!(f.range(0, 3), 9);
    f.add(1, -1); // set a[1] = 2
    assert_eq!(f.range(0, 3), 8);
    assert_eq!(count_inversions(&[2, 4, 1, 3, 5]), 3);
    println!("ok");
}
