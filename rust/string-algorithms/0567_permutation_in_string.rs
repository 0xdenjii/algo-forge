// LeetCode 567 - Permutation in String
// Return true if some permutation of s1 is a substring of s2 (lowercase letters).
// Approach: fixed-length sliding window comparing 26-letter count arrays.
// Time: O(26 * n), Space: O(1)

pub fn check_inclusion(s1: &str, s2: &str) -> bool {
    let (a, b) = (s1.as_bytes(), s2.as_bytes());
    let m = a.len();
    if m > b.len() {
        return false;
    }
    let idx = |c: u8| (c - b'a') as usize;
    let (mut need, mut win) = ([0u32; 26], [0u32; 26]);
    for i in 0..m {
        need[idx(a[i])] += 1;
        win[idx(b[i])] += 1;
    }
    if need == win {
        return true;
    }
    for i in m..b.len() {
        win[idx(b[i])] += 1;
        win[idx(b[i - m])] -= 1;
        if need == win {
            return true;
        }
    }
    false
}

fn main() {
    assert!(check_inclusion("ab", "eidbaooo"));
    assert!(!check_inclusion("ab", "eidboaoo"));
    println!("ok");
}
