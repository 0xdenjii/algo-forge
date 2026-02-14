// LeetCode 691 - Stickers to Spell Word
// Given stickers (words, each usable unlimited times, letters can be cut out),
// return the minimum number of stickers to spell target, or -1.
// Approach: bitmask DP over which target letters are covered; from each
// reachable state, greedily apply a sticker that covers the first missing letter.
// Time: O(2^t * S * t), Space: O(2^t)

pub fn min_stickers(stickers: &[&str], target: &str) -> i32 {
    let t = target.as_bytes();
    let n = t.len();
    let full = (1usize << n) - 1;
    let counts: Vec<[u8; 26]> = stickers
        .iter()
        .map(|s| {
            let mut c = [0u8; 26];
            s.bytes().for_each(|b| c[(b - b'a') as usize] += 1);
            c
        })
        .collect();
    let mut dp = vec![-1i32; full + 1];
    dp[0] = 0;
    for mask in 0..=full {
        if dp[mask] < 0 {
            continue;
        }
        let first = match (0..n).find(|&i| mask & (1 << i) == 0) {
            Some(f) => f,
            None => continue,
        };
        for c in &counts {
            if c[(t[first] - b'a') as usize] == 0 {
                continue;
            }
            let mut avail = *c;
            let mut next = mask;
            for i in 0..n {
                let k = (t[i] - b'a') as usize;
                if next & (1 << i) == 0 && avail[k] > 0 {
                    avail[k] -= 1;
                    next |= 1 << i;
                }
            }
            if dp[next] < 0 || dp[next] > dp[mask] + 1 {
                dp[next] = dp[mask] + 1;
            }
        }
    }
    dp[full]
}

fn main() {
    assert_eq!(min_stickers(&["with", "example", "science"], "thehat"), 3);
    assert_eq!(min_stickers(&["notice", "possible"], "basicbasic"), -1);
    println!("ok");
}
