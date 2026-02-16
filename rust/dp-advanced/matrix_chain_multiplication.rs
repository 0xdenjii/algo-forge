// Classic - Matrix Chain Multiplication
// Given dims p of length n+1 where matrix i is p[i] x p[i+1], find the minimum
// number of scalar multiplications to compute the product, and a parenthesization.
// Approach: interval DP cost[i][j] = min_k cost[i][k] + cost[k+1][j] + p[i]p[k+1]p[j+1],
// with split table for reconstruction.
// Time: O(n^3), Space: O(n^2)

pub fn matrix_chain(p: &[u64]) -> (u64, String) {
    let n = p.len() - 1;
    let mut cost = vec![vec![0u64; n]; n];
    let mut split = vec![vec![0usize; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            cost[i][j] = u64::MAX;
            for k in i..j {
                let c = cost[i][k] + cost[k + 1][j] + p[i] * p[k + 1] * p[j + 1];
                if c < cost[i][j] {
                    cost[i][j] = c;
                    split[i][j] = k;
                }
            }
        }
    }
    fn build(s: &[Vec<usize>], i: usize, j: usize) -> String {
        if i == j {
            format!("A{}", i + 1)
        } else {
            format!("({}{})", build(s, i, s[i][j]), build(s, s[i][j] + 1, j))
        }
    }
    (cost[0][n - 1], build(&split, 0, n - 1))
}

fn main() {
    let (c, order) = matrix_chain(&[40, 20, 30, 10, 30]);
    assert_eq!(c, 26000);
    assert_eq!(order, "((A1(A2A3))A4)");
    assert_eq!(matrix_chain(&[10, 20, 30]).0, 6000);
    println!("{c} {order}");
}
