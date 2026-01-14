// LeetCode 76 - Minimum Window Substring
// Return the smallest substring of s containing every character of t (with multiplicity), or "".
// Approach: sliding window with a need-count table and a "missing" counter.
// Time: O(|s| + |t|), Space: O(1)

pub fn min_window(s: &str, t: &str) -> String {
    let (sb, tb) = (s.as_bytes(), t.as_bytes());
    let mut need = [0i32; 256];
    for &c in tb {
        need[c as usize] += 1;
    }
    let mut missing = tb.len();
    let mut left = 0usize;
    let mut best: Option<(usize, usize)> = None;
    for (right, &c) in sb.iter().enumerate() {
        if need[c as usize] > 0 {
            missing -= 1;
        }
        need[c as usize] -= 1;
        while missing == 0 {
            if best.map_or(true, |(l, r)| right + 1 - left < r - l) {
                best = Some((left, right + 1));
            }
            let lc = sb[left] as usize;
            need[lc] += 1;
            if need[lc] > 0 {
                missing += 1;
            }
            left += 1;
        }
    }
    best.map_or(String::new(), |(l, r)| s[l..r].to_string())
}

fn main() {
    assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
    assert_eq!(min_window("a", "a"), "a");
    assert_eq!(min_window("a", "aa"), "");
    println!("ok");
}
