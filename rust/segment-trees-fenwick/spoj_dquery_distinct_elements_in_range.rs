// SPOJ DQUERY - D-query (distinct elements in a range)
// Problem: Given an array and queries (l, r), return the number of distinct
//   values in a[l..=r].
// Approach: Offline. Sort the queries by right end and sweep i from left to
//   right. Keep a 1 only at the most recent occurrence of each value: when
//   a[i] reappears, remove the mark at its previous index and add one at i.
//   Every query with r == i is then answered by the Fenwick range sum over
//   [l, r].
// Complexity: O((n + q) log n) time, O(n + q) space.

use std::collections::HashMap;

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize, d: i32) {
        while i < self.0.len() {
            self.0[i] += d;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i &= i - 1;
        }
        s
    }
}

fn distinct_queries(a: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    let mut order: Vec<usize> = (0..queries.len()).collect();
    order.sort_by_key(|&q| queries[q].1);
    let mut bit = Fenwick::new(a.len());
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut ans = vec![0; queries.len()];
    let mut next = 0;
    for (i, &v) in a.iter().enumerate() {
        if let Some(prev) = last.insert(v, i) {
            bit.add(prev + 1, -1);
        }
        bit.add(i + 1, 1);
        while next < order.len() && queries[order[next]].1 == i {
            let (l, r) = queries[order[next]];
            ans[order[next]] = bit.prefix(r + 1) - bit.prefix(l);
            next += 1;
        }
    }
    ans
}

fn main() {
    // Official sample (1-based): (1,5)->3, (2,4)->2, (3,5)->3
    let a = [1, 1, 2, 1, 3];
    assert_eq!(distinct_queries(&a, &[(0, 4), (1, 3), (2, 4)]), vec![3, 2, 3]);

    let b = [4, 2, 4, 4, 7, 2, 9, 7];
    let qs: Vec<(usize, usize)> = (0..b.len()).flat_map(|l| (l..b.len()).map(move |r| (l, r))).collect();
    let got = distinct_queries(&b, &qs);
    for (k, &(l, r)) in qs.iter().enumerate() {
        let mut w = b[l..=r].to_vec();
        w.sort_unstable();
        w.dedup();
        assert_eq!(got[k], w.len() as i32);
    }
    println!("DQUERY ok");
}
