// LeetCode 308 - Range Sum Query 2D - Mutable
// Problem: Given a 2D matrix, support update(row, col, val) and
//   sum_region(r1, c1, r2, c2), which returns the sum of the sub-rectangle
//   with corners (r1, c1) and (r2, c2), inclusive.
// Approach: 2D Fenwick tree. Rectangle sums use inclusion-exclusion over
//   four prefix-rectangle queries.
// Complexity: update/query O(log R * log C); build O(R C log R log C); space O(R C).

struct NumMatrix {
    bit: Vec<Vec<i64>>,
    vals: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let r = matrix.len();
        let c = matrix.first().map_or(0, |row| row.len());
        let mut m = NumMatrix { bit: vec![vec![0; c + 1]; r + 1], vals: vec![vec![0; c]; r] };
        for (i, row) in matrix.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                m.update(i, j, v);
            }
        }
        m
    }

    fn add(&mut self, row: usize, col: usize, delta: i64) {
        let mut i = row + 1;
        while i < self.bit.len() {
            let mut j = col + 1;
            while j < self.bit[i].len() {
                self.bit[i][j] += delta;
                j += j & j.wrapping_neg();
            }
            i += i & i.wrapping_neg();
        }
    }

    /// Sum over the rectangle [0, row) x [0, col).
    fn prefix(&self, row: usize, col: usize) -> i64 {
        let mut s = 0;
        let mut i = row;
        while i > 0 {
            let mut j = col;
            while j > 0 {
                s += self.bit[i][j];
                j &= j - 1;
            }
            i &= i - 1;
        }
        s
    }

    fn update(&mut self, row: usize, col: usize, val: i32) {
        let delta = val as i64 - self.vals[row][col] as i64;
        self.vals[row][col] = val;
        self.add(row, col, delta);
    }

    fn sum_region(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        (self.prefix(r2 + 1, c2 + 1) - self.prefix(r1, c2 + 1) - self.prefix(r2 + 1, c1)
            + self.prefix(r1, c1)) as i32
    }
}

fn main() {
    let mut m = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    assert_eq!(m.sum_region(2, 1, 4, 3), 8);
    m.update(3, 2, 2);
    assert_eq!(m.sum_region(2, 1, 4, 3), 10);
    println!("0308 ok");
}
