// Problem 875: Koko Eating Bananas
// Piles of bananas and h hours; each hour Koko eats up to k bananas from a single pile.
// Find the minimum integer speed k that lets her finish within h hours.
// Approach: Binary search on the answer: feasibility is monotone in k; search k in [1, max].
// Complexity: O(n log max) time, O(1) space.
pub fn min_eating_speed(piles: &[u64], h: u64) -> u64 {
    let ok = |k: u64| piles.iter().map(|&p| p.div_ceil(k)).sum::<u64>() <= h;
    let (mut lo, mut hi) = (1u64, *piles.iter().max().expect("non-empty piles"));
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ok(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), 23);
    println!("ok");
}
