// LeetCode 875 - Koko Eating Bananas
// piles[i] bananas; each hour Koko eats up to k bananas from one pile. Return
// the minimum integer k such that she finishes all piles within h hours.
// Approach: binary search on the answer k in [1, max(piles)]; hours(k) is monotone.
// Time: O(n log max), Space: O(1)

fn min_eating_speed(piles: &[i32], h: i32) -> i32 {
    let hours = |k: i64| piles.iter().map(|&p| (p as i64 + k - 1) / k).sum::<i64>();
    let (mut lo, mut hi) = (1i64, *piles.iter().max().unwrap() as i64);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if hours(mid) <= h as i64 { hi = mid } else { lo = mid + 1 }
    }
    lo as i32
}

fn main() {
    assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), 23);
    println!("ok");
}
