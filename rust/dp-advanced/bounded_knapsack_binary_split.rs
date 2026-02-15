// Classic - Bounded Knapsack (binary splitting)
// Items have weight w, value v and available count c. Maximize total value
// with total weight <= capacity W.
// Approach: split each count into powers of two (1,2,4,...,rest), turning the
// problem into 0/1 knapsack over O(sum log c) pseudo-items.
// Time: O(W * sum log c), Space: O(W)

pub fn bounded_knapsack(items: &[(usize, u64, usize)], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &(w, v, mut c) in items {
        let mut k = 1;
        while c > 0 {
            let take = k.min(c);
            c -= take;
            k <<= 1;
            let (bw, bv) = (w * take, v * take as u64);
            if bw > cap {
                continue;
            }
            for x in (bw..=cap).rev() {
                dp[x] = dp[x].max(dp[x - bw] + bv);
            }
        }
    }
    dp[cap]
}

fn main() {
    // (weight, value, count)
    let items = [(1, 2, 3), (3, 7, 2), (4, 9, 1)];
    assert_eq!(bounded_knapsack(&items, 7), 16); // 3+3+1 -> 7+7+2
    assert_eq!(bounded_knapsack(&items, 0), 0);
    println!("ok");
}
