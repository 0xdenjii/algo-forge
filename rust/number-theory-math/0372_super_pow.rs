// LeetCode 372 - Super Pow
// Compute a^b mod 1337 where a is a positive integer and b is a huge integer given as a digit array.
// Approach: a^(10q + d) = (a^q)^10 * a^d; fold over the digits using fast modular exponentiation.
// Time: O(len(b)), Space: O(1)
const MOD: u64 = 1337;

fn pow_mod(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn super_pow(a: u64, b: &[u64]) -> u64 {
    b.iter().fold(1, |acc, &d| pow_mod(acc, 10) * pow_mod(a, d) % MOD)
}

fn main() {
    assert_eq!(super_pow(2, &[3]), 8);
    assert_eq!(super_pow(2, &[1, 0]), 1024);
    assert_eq!(super_pow(2_147_483_647, &[2, 0, 0]), 1198);
    println!("ok");
}
