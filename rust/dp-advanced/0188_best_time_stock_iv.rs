// LeetCode 188 - Best Time to Buy and Sell Stock IV
// Maximum profit with at most k transactions given daily prices.
// Approach: buy[t]/sell[t] state arrays; if k >= n/2 the limit never binds,
// so sum every positive day-to-day gain.
// Time: O(n*k), Space: O(k)

pub fn max_profit(k: usize, prices: &[i32]) -> i32 {
    let n = prices.len();
    if k == 0 || n < 2 {
        return 0;
    }
    if k >= n / 2 {
        return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
    }
    let mut buy = vec![i32::MIN / 2; k + 1];
    let mut sell = vec![0; k + 1];
    for &p in prices {
        for t in 1..=k {
            buy[t] = buy[t].max(sell[t - 1] - p);
            sell[t] = sell[t].max(buy[t] + p);
        }
    }
    sell[k]
}

fn main() {
    assert_eq!(max_profit(2, &[2, 4, 1]), 2);
    assert_eq!(max_profit(2, &[3, 2, 6, 5, 0, 3]), 7);
    println!("ok");
}
