// Title: Sparse Table for Range Minimum Query
// Problem: Static array; answer many queries min(a[l..=r]) in O(1) each.
// Approach: st[k][i] = min of a[i..i + 2^k); a query covers [l, r] with two overlapping
// power-of-two blocks (valid because min is idempotent).
// Complexity: O(n log n) build and space, O(1) per query.
pub struct SparseTable {
    st: Vec<Vec<i64>>,
}

impl SparseTable {
    pub fn new(a: &[i64]) -> Self {
        let mut st = vec![a.to_vec()];
        let mut k = 1;
        while (1 << k) <= a.len() {
            let prev = &st[k - 1];
            let half = 1 << (k - 1);
            let row = (0..=a.len() - (1 << k)).map(|i| prev[i].min(prev[i + half])).collect();
            st.push(row);
            k += 1;
        }
        Self { st }
    }
    pub fn min(&self, l: usize, r: usize) -> i64 {
        let k = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
        self.st[k][l].min(self.st[k][r + 1 - (1 << k)])
    }
}

fn main() {
    let s = SparseTable::new(&[5, 2, 4, 7, 1, 3, 6]);
    assert_eq!(s.min(0, 3), 2);
    assert_eq!(s.min(2, 6), 1);
    assert_eq!(s.min(5, 6), 3);
    assert_eq!(s.min(3, 3), 7);
    println!("ok");
}
