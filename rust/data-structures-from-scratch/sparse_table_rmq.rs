// Classic - Sparse Table for Range Minimum Query
// Given a static array, answer min(a[l..=r]) queries in O(1) after preprocessing.
// Approach: table[k][i] = min of a[i .. i + 2^k); a query covers [l, r] with two
// overlapping power-of-two windows (min is idempotent, so overlap is fine).
// Time: O(n log n) build, O(1) query. Space: O(n log n).
pub struct SparseTable<T: Ord + Copy> {
    table: Vec<Vec<T>>,
}

impl<T: Ord + Copy> SparseTable<T> {
    pub fn new(a: &[T]) -> Self {
        let mut table = vec![a.to_vec()];
        let mut k = 1;
        while (1 << k) <= a.len() {
            let prev = &table[k - 1];
            let half = 1 << (k - 1);
            let row: Vec<T> = (0..=a.len() - (1 << k)).map(|i| prev[i].min(prev[i + half])).collect();
            table.push(row);
            k += 1;
        }
        Self { table }
    }

    /// Minimum over the inclusive range [l, r]. Panics if l > r or out of range.
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        let k = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
        self.table[k][l].min(self.table[k][r + 1 - (1 << k)])
    }
}

fn main() {
    let a = [5, 2, 4, 7, 6, 3, 1, 2];
    let st = SparseTable::new(&a);
    assert_eq!(st.query(0, 7), 1);
    assert_eq!(st.query(0, 2), 2);
    assert_eq!(st.query(2, 4), 4);
    assert_eq!(st.query(3, 3), 7);
    for l in 0..a.len() {
        for r in l..a.len() {
            assert_eq!(st.query(l, r), *a[l..=r].iter().min().unwrap());
        }
    }
    println!("sparse table ok");
}
