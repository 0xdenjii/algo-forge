// Title: Matrix Exponentiation (n-th Fibonacci mod p for n up to 1e18)
// Problem: Compute F(n) mod 1e9+7 for very large n (CSES Fibonacci Numbers).
// Approach: [[1,1],[1,0]]^n = [[F(n+1),F(n)],[F(n),F(n-1)]] via binary exponentiation; the
// generic square-matrix multiply works for any linear recurrence.
// Complexity: O(k^3 log n) time with k = 2, O(k^2) space.
const MOD: u64 = 1_000_000_007;
type Mat = Vec<Vec<u64>>;

fn mul(a: &Mat, b: &Mat) -> Mat {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..n {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

pub fn mat_pow(mut m: Mat, mut e: u64) -> Mat {
    let n = m.len();
    let mut r: Mat = (0..n).map(|i| (0..n).map(|j| (i == j) as u64).collect()).collect();
    while e > 0 {
        if e & 1 == 1 {
            r = mul(&r, &m);
        }
        m = mul(&m, &m);
        e >>= 1;
    }
    r
}

pub fn fib(n: u64) -> u64 {
    mat_pow(vec![vec![1, 1], vec![1, 0]], n)[0][1]
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(90), 2_880_067_194_370_816_120 % MOD);
    println!("F(1e18) mod p = {}", fib(1_000_000_000_000_000_000));
}
