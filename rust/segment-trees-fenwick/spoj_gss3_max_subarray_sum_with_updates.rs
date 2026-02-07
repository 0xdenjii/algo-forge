// SPOJ GSS3 - Can you answer these queries III
// Problem: Given an array, support set(i, v) and max_sub(l, r): the maximum
//   sum of a non-empty contiguous subarray inside a[l..=r].
// Approach: Each segment tree node stores (sum, best prefix, best suffix,
//   best subarray). The merge is:
//     best = max(L.best, R.best, L.suf + R.pre),
//     pre = max(L.pre, L.sum + R.pre), suf = max(R.suf, R.sum + L.suf).
//   The merge is not commutative, so a query folds the left and right
//   accumulators separately.
// Complexity: O(log n) per operation, O(n) space.

#[derive(Clone, Copy, Debug)]
struct Info {
    sum: i64,
    pre: i64,
    suf: i64,
    best: i64,
}

impl Info {
    fn leaf(v: i64) -> Self {
        Info { sum: v, pre: v, suf: v, best: v }
    }
}

/// None acts as the identity element.
fn merge(a: Option<Info>, b: Option<Info>) -> Option<Info> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(l), Some(r)) => Some(Info {
            sum: l.sum + r.sum,
            pre: l.pre.max(l.sum + r.pre),
            suf: r.suf.max(r.sum + l.suf),
            best: l.best.max(r.best).max(l.suf + r.pre),
        }),
    }
}

struct Gss {
    size: usize,
    t: Vec<Option<Info>>,
}

impl Gss {
    fn new(a: &[i64]) -> Self {
        let size = a.len().next_power_of_two();
        let mut t = vec![None; 2 * size];
        for (i, &v) in a.iter().enumerate() {
            t[size + i] = Some(Info::leaf(v));
        }
        for i in (1..size).rev() {
            t[i] = merge(t[2 * i], t[2 * i + 1]);
        }
        Gss { size, t }
    }

    fn set(&mut self, idx: usize, v: i64) {
        let mut i = idx + self.size;
        self.t[i] = Some(Info::leaf(v));
        while i > 1 {
            i >>= 1;
            self.t[i] = merge(self.t[2 * i], self.t[2 * i + 1]);
        }
    }

    fn max_sub(&self, l: usize, r: usize) -> i64 {
        let (mut l, mut r) = (l + self.size, r + self.size + 1);
        let (mut left, mut right) = (None, None);
        while l < r {
            if l & 1 == 1 {
                left = merge(left, self.t[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = merge(self.t[r], right);
            }
            l >>= 1;
            r >>= 1;
        }
        merge(left, right).expect("non-empty range").best
    }
}

fn main() {
    // Official sample (1-based): a = [1, 2, 3, 4]
    // "1 1 3" -> 6, "0 3 -3" (a3 = -3), "1 2 4" -> 4, "1 3 3" -> -3
    let mut g = Gss::new(&[1, 2, 3, 4]);
    assert_eq!(g.max_sub(0, 2), 6);
    g.set(2, -3);
    assert_eq!(g.max_sub(1, 3), 4);
    assert_eq!(g.max_sub(2, 2), -3);

    // Cross-check with brute-force Kadane over every range.
    let a = [-2i64, 1, -3, 4, -1, 2, 1, -5, 4];
    let g = Gss::new(&a);
    for l in 0..a.len() {
        for r in l..a.len() {
            let mut best = i64::MIN;
            for i in l..=r {
                let mut s = 0;
                for j in i..=r {
                    s += a[j];
                    best = best.max(s);
                }
            }
            assert_eq!(g.max_sub(l, r), best);
        }
    }
    println!("GSS3 ok");
}
