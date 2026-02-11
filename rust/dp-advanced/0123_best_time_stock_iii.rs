// LeetCode 123 - Best Time to Buy and Sell Stock III
// Given daily prices, find the maximum profit with at most two transactions
// (you must sell before buying again).
// Approach: four-state machine: buy1, sell1, buy2, sell2.
// Time: O(n), Space: O(1)

pub fn max_profit(prices: &[i32]) -> i32 {
    let (mut b1, mut s1, mut b2, mut s2) = (i32::MIN / 2, 0, i32::MIN / 2, 0);
    for &p in prices {
        b1 = b1.max(-p);
        s1 = s1.max(b1 + p);
        b2 = b2.max(s1 - p);
        s2 = s2.max(b2 + p);
    }
    s2
}

fn main() {
    assert_eq!(max_profit(&[3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    println!("ok");
}
