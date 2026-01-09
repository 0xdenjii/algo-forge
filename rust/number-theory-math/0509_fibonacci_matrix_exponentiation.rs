// LeetCode 509 - Fibonacci Number (extended: huge n modulo 1e9+7)
// Compute F(n) mod 1_000_000_007 for n up to 10^18, with F(0) = 0, F(1) = 1.
// Approach: [[1,1],[1,0]]^n = [[F(n+1), F(n)], [F(n), F(n-1)]]; raise by binary exponentiation.
// Time: O(log n) (8 multiplications per step), Space: O(1)
const MOD: u64 = 1_000_000_007;
type Mat = [[u64; 2]; 2];

fn mul(a: &Mat, b: &Mat) -> Mat {
    let mut c = [[0u64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j]) % MOD;
        }
    }
    c
}

fn fib(mut n: u64) -> u64 {
    let mut result: Mat = [[1, 0], [0, 1]];
    let mut base: Mat = [[1, 1], [1, 0]];
    while n > 0 {
        if n & 1 == 1 {
            result = mul(&result, &base);
        }
        base = mul(&base, &base);
        n >>= 1;
    }
    result[0][1]
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(90) , 2_880_067_194_370_816_120 % MOD);
    println!("F(10^18) mod p = {}", fib(1_000_000_000_000_000_000));
}
