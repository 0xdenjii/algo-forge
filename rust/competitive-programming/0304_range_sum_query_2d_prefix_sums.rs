// Problem 304: Range Sum Query 2D - Immutable
// Given a matrix, answer many queries for the sum of the sub-rectangle (r1,c1)-(r2,c2)
// inclusive.
// Approach: P[i+1][j+1] = sum of matrix[0..=i][0..=j]; inclusion-exclusion per query.
// Complexity: O(RC) build, O(1) per query; O(RC) space.
pub struct NumMatrix {
    p: Vec<Vec<i64>>,
}

impl NumMatrix {
    pub fn new(m: &[Vec<i64>]) -> Self {
        let (r, c) = (m.len(), m.first().map_or(0, |x| x.len()));
        let mut p = vec![vec![0; c + 1]; r + 1];
        for i in 0..r {
            for j in 0..c {
                p[i + 1][j + 1] = m[i][j] + p[i][j + 1] + p[i + 1][j] - p[i][j];
            }
        }
        Self { p }
    }
    pub fn sum_region(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
        let p = &self.p;
        p[r2 + 1][c2 + 1] - p[r1][c2 + 1] - p[r2 + 1][c1] + p[r1][c1]
    }
}

fn main() {
    let m = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let nm = NumMatrix::new(&m);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
    assert_eq!(nm.sum_region(1, 1, 2, 2), 11);
    assert_eq!(nm.sum_region(1, 2, 2, 4), 12);
    println!("ok");
}
