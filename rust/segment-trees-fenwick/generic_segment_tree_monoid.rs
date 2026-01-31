// Classic - Generic Iterative Segment Tree over a Monoid
// Problem: Write one reusable point-update / range-fold segment tree that
//   works for any associative operation with an identity (sum, min, gcd,
//   matrix product, string concatenation, ...), including non-commutative ones.
// Approach: A Monoid trait gives the identity and the combine function. The
//   tree is the bottom-up (AtCoder-library style) layout over 2*size slots.
//   A query keeps separate left and right accumulators so the operand order
//   is preserved for non-commutative monoids.
// Complexity: build O(n), point set / range fold O(log n); space O(n).

trait Monoid {
    type T: Clone;
    fn identity() -> Self::T;
    fn combine(a: &Self::T, b: &Self::T) -> Self::T;
}

struct SegTree<M: Monoid> {
    size: usize,
    data: Vec<M::T>,
}

impl<M: Monoid> SegTree<M> {
    fn from_slice(v: &[M::T]) -> Self {
        let size = v.len().next_power_of_two();
        let mut data = vec![M::identity(); 2 * size];
        data[size..size + v.len()].clone_from_slice(v);
        for i in (1..size).rev() {
            data[i] = M::combine(&data[2 * i], &data[2 * i + 1]);
        }
        SegTree { size, data }
    }

    fn set(&mut self, idx: usize, value: M::T) {
        let mut i = idx + self.size;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            self.data[i] = M::combine(&self.data[2 * i], &self.data[2 * i + 1]);
        }
    }

    /// Fold over the half-open range [l, r).
    fn fold(&self, l: usize, r: usize) -> M::T {
        let (mut l, mut r) = (l + self.size, r + self.size);
        let (mut left, mut right) = (M::identity(), M::identity());
        while l < r {
            if l & 1 == 1 {
                left = M::combine(&left, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = M::combine(&self.data[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::combine(&left, &right)
    }
}

struct Sum;
impl Monoid for Sum {
    type T = i64;
    fn identity() -> i64 { 0 }
    fn combine(a: &i64, b: &i64) -> i64 { a + b }
}

struct Min;
impl Monoid for Min {
    type T = i64;
    fn identity() -> i64 { i64::MAX }
    fn combine(a: &i64, b: &i64) -> i64 { *a.min(b) }
}

struct Gcd;
impl Monoid for Gcd {
    type T = u64;
    fn identity() -> u64 { 0 }
    fn combine(a: &u64, b: &u64) -> u64 {
        let (mut x, mut y) = (*a, *b);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        x
    }
}

/// Non-commutative: string concatenation checks that the order is preserved.
struct Concat;
impl Monoid for Concat {
    type T = String;
    fn identity() -> String { String::new() }
    fn combine(a: &String, b: &String) -> String { format!("{a}{b}") }
}

fn main() {
    let mut sum = SegTree::<Sum>::from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(sum.fold(1, 4), 9);
    sum.set(2, 10);
    assert_eq!(sum.fold(0, 5), 22);

    let min = SegTree::<Min>::from_slice(&[7, 3, 9, -2, 8]);
    assert_eq!(min.fold(0, 3), 3);
    assert_eq!(min.fold(0, 5), -2);

    let gcd = SegTree::<Gcd>::from_slice(&[12, 18, 24, 7]);
    assert_eq!(gcd.fold(0, 3), 6);
    assert_eq!(gcd.fold(0, 4), 1);

    let words: Vec<String> = "a b c d e f g".split(' ').map(String::from).collect();
    let mut cat = SegTree::<Concat>::from_slice(&words);
    assert_eq!(cat.fold(1, 6), "bcdef");
    cat.set(3, "X".to_string());
    assert_eq!(cat.fold(0, 7), "abcXefg");
    println!("generic monoid segment tree ok");
}
