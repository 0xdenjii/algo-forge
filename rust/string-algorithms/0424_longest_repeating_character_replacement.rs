// LeetCode 424 - Longest Repeating Character Replacement
// Given an uppercase string s and integer k, you may replace at most k characters.
// Return the length of the longest substring containing a single repeated letter afterwards.
// Approach: sliding window tracking the max frequency seen; window is valid while
// len - max_freq <= k. max_freq never needs to decrease (the answer only grows with it).
// Time: O(n), Space: O(1)

pub fn character_replacement(s: &str, k: usize) -> usize {
    let b = s.as_bytes();
    let mut cnt = [0usize; 26];
    let (mut left, mut max_freq, mut best) = (0, 0, 0);
    for right in 0..b.len() {
        let c = (b[right] - b'A') as usize;
        cnt[c] += 1;
        max_freq = max_freq.max(cnt[c]);
        while right + 1 - left - max_freq > k {
            cnt[(b[left] - b'A') as usize] -= 1;
            left += 1;
        }
        best = best.max(right + 1 - left);
    }
    best
}

fn main() {
    assert_eq!(character_replacement("ABAB", 2), 4);
    assert_eq!(character_replacement("AABABBA", 1), 4);
    assert_eq!(character_replacement("", 3), 0);
    println!("ok");
}
