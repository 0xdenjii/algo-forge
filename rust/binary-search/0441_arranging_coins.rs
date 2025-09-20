// LeetCode 441 - Arranging Coins
// With n coins build a staircase where row i has exactly i coins. Return the
// number of complete rows.
// Approach: largest k with k(k+1)/2 <= n, found by binary search in u64.
// Time: O(log n), Space: O(1)

fn arrange_coins(n: i32) -> i32 {
    let n = n as u64;
    let (mut lo, mut hi) = (0u64, 1u64 << 17); // k(k+1)/2 > i32::MAX for k = 2^17
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if mid * (mid + 1) / 2 <= n { lo = mid } else { hi = mid - 1 }
    }
    lo as i32
}

fn main() {
    assert_eq!(arrange_coins(5), 2);
    assert_eq!(arrange_coins(8), 3);
    assert_eq!(arrange_coins(1), 1);
    assert_eq!(arrange_coins(i32::MAX), 65535);
    println!("ok");
}
