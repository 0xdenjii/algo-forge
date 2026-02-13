// LeetCode 629 - K Inverse Pairs Array
// Count permutations of 1..=n with exactly k inverse pairs, modulo 1e9+7.
// Approach: dp[i][j] = sum_{t=0..min(j,i-1)} dp[i-1][j-t]; computed with a
// sliding-window prefix sum over the previous row.
// Time: O(n*k), Space: O(k)

const MOD: i64 = 1_000_000_007;

pub fn k_inverse_pairs(n: usize, k: usize) -> i64 {
    let mut dp = vec![0i64; k + 1];
    dp[0] = 1;
    for i in 2..=n {
        let mut nd = vec![0i64; k + 1];
        let mut window = 0i64;
        for j in 0..=k {
            window = (window + dp[j]) % MOD;
            if j >= i {
                window = (window - dp[j - i] + MOD) % MOD;
            }
            nd[j] = window;
        }
        dp = nd;
    }
    dp[k]
}

fn main() {
    assert_eq!(k_inverse_pairs(3, 0), 1);
    assert_eq!(k_inverse_pairs(3, 1), 2);
    assert_eq!(k_inverse_pairs(1000, 1000), 663677020);
    println!("ok");
}
