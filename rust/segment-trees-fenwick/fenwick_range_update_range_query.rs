// Classic - Fenwick Tree with Range Update and Range Query
// Problem: Maintain an array of n integers (initially 0) and support:
//   add(l, r, v): add v to every a[l..=r], and
//   sum(l, r): return a[l] + ... + a[r].
// Approach: Two Fenwick trees B1 and B2 over the difference array. The prefix
//   sum is prefix(i) = sum_B1(i) * i - sum_B2(i). A range add makes four point
//   updates: B1[l] += v, B1[r+1] -= v, B2[l] += v*(l-1), B2[r+1] -= v*r.
// Complexity: O(log n) per operation, O(n) space.

struct RangeFenwick {
    b1: Vec<i64>,
    b2: Vec<i64>,
}

impl RangeFenwick {
    fn new(n: usize) -> Self {
        RangeFenwick { b1: vec![0; n + 2], b2: vec![0; n + 2] }
    }

    fn point(tree: &mut [i64], mut i: usize, v: i64) {
        while i < tree.len() {
            tree[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    fn sum_tree(tree: &[i64], mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += tree[i];
            i &= i - 1;
        }
        s
    }

    /// 1-based inclusive range add.
    fn add(&mut self, l: usize, r: usize, v: i64) {
        Self::point(&mut self.b1, l, v);
        Self::point(&mut self.b1, r + 1, -v);
        Self::point(&mut self.b2, l, v * (l as i64 - 1));
        Self::point(&mut self.b2, r + 1, -v * r as i64);
    }

    fn prefix(&self, i: usize) -> i64 {
        Self::sum_tree(&self.b1, i) * i as i64 - Self::sum_tree(&self.b2, i)
    }

    /// 1-based inclusive range sum.
    fn sum(&self, l: usize, r: usize) -> i64 {
        self.prefix(r) - self.prefix(l - 1)
    }
}

fn main() {
    // Randomized cross-check against a plain array (deterministic LCG).
    let n = 50;
    let mut fw = RangeFenwick::new(n);
    let mut naive = vec![0i64; n + 1];
    let mut seed: u64 = 12345;
    let mut rnd = |m: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % m
    };
    for _ in 0..2000 {
        let a = rnd(n as u64) as usize + 1;
        let b = rnd(n as u64) as usize + 1;
        let (l, r) = (a.min(b), a.max(b));
        if rnd(2) == 0 {
            let v = rnd(200) as i64 - 100;
            fw.add(l, r, v);
            for x in &mut naive[l..=r] {
                *x += v;
            }
        } else {
            assert_eq!(fw.sum(l, r), naive[l..=r].iter().sum::<i64>());
        }
    }
    println!("fenwick range-update range-query ok");
}
