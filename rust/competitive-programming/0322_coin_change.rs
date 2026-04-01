// Problem 322: Coin Change
// Given coin denominations (unlimited supply of each) and an amount, return the fewest coins
// that sum to the amount, or -1 if it cannot be made.
// Approach: Unbounded-knapsack DP over amounts, using Option to represent "unreachable".
// Complexity: O(amount * coins) time, O(amount) space.
pub fn coin_change(coins: &[usize], amount: usize) -> i32 {
    let mut dp: Vec<Option<u32>> = vec![None; amount + 1];
    dp[0] = Some(0);
    for a in 1..=amount {
        dp[a] = coins
            .iter()
            .filter(|&&c| c <= a)
            .filter_map(|&c| dp[a - c])
            .min()
            .map(|m| m + 1);
    }
    dp[amount].map_or(-1, |v| v as i32)
}

fn main() {
    assert_eq!(coin_change(&[1, 2, 5], 11), 3);
    assert_eq!(coin_change(&[2], 3), -1);
    assert_eq!(coin_change(&[1], 0), 0);
    println!("ok");
}
